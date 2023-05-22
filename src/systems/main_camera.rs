use {
    crate::{
        components::{main_camera::*, player::Player},
        constants::main_camera::*,
        prelude::*,
    },
    bevy::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub fn spawn_main_camera(mut cmds: Commands) {
    cmds.spawn((MainCamera, Camera2dBundle::default()));
}

pub fn follow_player(
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
}

pub fn adjust_zoom(
    mut cam_qry: Query<&mut OrthographicProjection, With<MainCamera>>,
    action_state_qry: Query<&ActionState<Action>, With<Player>>,
    time: Res<Time>,
) {
    let mut projection = cam_qry.single_mut();
    let action_state = action_state_qry.single();
    let dt = time.delta_seconds();

    if action_state.pressed(Action::ZoomIn) {
        projection.scale = f32::max(
            projection.scale - CAMERA_ZOOM_AMOUNT * dt,
            CAMERA_ZOOM_IN_LIMIT,
        );
    }
    if action_state.pressed(Action::ZoomOut) {
        projection.scale = f32::min(
            projection.scale + CAMERA_ZOOM_AMOUNT * dt,
            CAMERA_ZOOM_OUT_LIMIT,
        );
    }
}
