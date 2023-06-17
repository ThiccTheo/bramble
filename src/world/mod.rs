pub mod main_camera;
pub mod player;
pub mod tile;
pub mod world_generation;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    main_camera::MainCameraPlugin,
    player::PlayerPlugin,
    tile::TilePlugin,
    world_generation::WorldGenerationPlugin,
};

pub struct WorldPlugins;

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(MainCameraPlugin)
            .add(WorldGenerationPlugin)
            .add(TilePlugin)
    }
}
