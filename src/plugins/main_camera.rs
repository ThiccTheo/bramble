use {
    crate::{prelude::*, systems::main_camera::*},
    bevy::prelude::*,
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_main_camera)
            .add_systems((follow_player, adjust_zoom).in_set(OnUpdate(GameState::Playing)));
    }
}
