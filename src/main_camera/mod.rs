pub mod data;
pub mod funcs;

use {crate::prelude::*, bevy::prelude::*, data::*, funcs::*};

const CAMERA_FOLLOW_SPEED: f32 = 10.;
const CAMERA_ZOOM_AMOUNT: f32 = 5.;
const DEFAULT_CAMERA_ZOOM: f32 = 1.;
const CAMERA_ZOOM_OFFSET: f32 = 0.4;
const CAMERA_ZOOM_IN_LIMIT: f32 = DEFAULT_CAMERA_ZOOM - CAMERA_ZOOM_OFFSET;
const CAMERA_ZOOM_OUT_LIMIT: f32 = DEFAULT_CAMERA_ZOOM + CAMERA_ZOOM_OFFSET;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_main_camera)
            .add_systems((follow_player, adjust_zoom).in_set(OnUpdate(GameState::Playing)));
    }
}
