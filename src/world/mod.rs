use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    main_camera::MainCameraPlugin,
    player::PlayerPlugin,
    world_generation::WorldGenerationPlugin,
};

pub mod main_camera;
pub mod player;
pub mod world_generation;

pub struct WorldPlugins;

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(MainCameraPlugin)
            .add(WorldGenerationPlugin)
    }
}
