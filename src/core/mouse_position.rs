use {
    crate::{core::game_state::GameState, world::main_camera::MainCamera},
    bevy::{prelude::*, window::PrimaryWindow},
};

pub(super) struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .add_system(update_mouse_position.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

fn update_mouse_position(
    mut old_mouse_pos: ResMut<MousePosition>,
    win_qry: Query<&Window, With<PrimaryWindow>>,
    cam_qry: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let win = win_qry.single();
    let (cam, cam_transform) = cam_qry.single();
    let Some(new_mouse_pos) = win.cursor_position().and_then(|mouse_pos| cam.viewport_to_world_2d(cam_transform, mouse_pos)) else { return };
    old_mouse_pos.0 = new_mouse_pos;
}
