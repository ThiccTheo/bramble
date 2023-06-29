use {
    super::player::{self, Player, PlayerControl},
    crate::states::game_state::GameState,
    bevy::prelude::*,
    leafwing_input_manager::prelude::*,
};

const CAMERA_FOLLOW_SPEED: f32 = 10.;
const CAMERA_ZOOM_AMOUNT: f32 = 5.;
const DEFAULT_CAMERA_ZOOM: f32 = 1.;
const CAMERA_ZOOM_OFFSET: f32 = 0.9;
const CAMERA_ZOOM_IN_LIMIT: f32 = DEFAULT_CAMERA_ZOOM - CAMERA_ZOOM_OFFSET;
const CAMERA_ZOOM_OUT_LIMIT: f32 = DEFAULT_CAMERA_ZOOM + CAMERA_ZOOM_OFFSET;
const DEFAULT_CAMERA_Z: f32 = 999.9;

pub(super) struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_camera.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (follow_player.after(player::move_player), adjust_zoom)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_main_camera(mut cmds: Commands) {
    cmds.spawn((MainCamera, Camera2dBundle::default()));
}

fn follow_player(
    mut cam_qry: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_qry: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    let mut cam_transform = cam_qry.single_mut();
    let player_transform = player_qry.single();
    let dt = time.delta_seconds();

    cam_transform.translation = Vec3::lerp(
        cam_transform.translation,
        player_transform.translation,
        CAMERA_FOLLOW_SPEED * dt,
    );
    cam_transform.translation.z = DEFAULT_CAMERA_Z;
}

fn adjust_zoom(
    mut cam_qry: Query<&mut OrthographicProjection, With<MainCamera>>,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    time: Res<Time>,
) {
    let mut projection = cam_qry.single_mut();
    let action_state = action_state_qry.single();
    let dt = time.delta_seconds();

    if action_state.pressed(PlayerControl::ZoomIn) {
        projection.scale = f32::max(
            projection.scale - CAMERA_ZOOM_AMOUNT * dt,
            CAMERA_ZOOM_IN_LIMIT,
        );
    }
    if action_state.pressed(PlayerControl::ZoomOut) {
        projection.scale = f32::min(
            projection.scale + CAMERA_ZOOM_AMOUNT * dt,
            CAMERA_ZOOM_OUT_LIMIT,
        );
    }
}
