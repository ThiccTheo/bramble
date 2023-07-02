pub mod block;
pub mod main_camera;
pub mod player;
pub mod wall;
pub mod world_generation;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    bevy_ecs_tilemap::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    block::BlockPlugin,
    main_camera::MainCameraPlugin,
    player::PlayerPlugin,
    world_generation::WorldGenerationPlugin,
};

pub struct WorldPlugins;

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WorldInspectorPlugin::default())
            .add(TilemapPlugin)
            .add(MainCameraPlugin)
            .add(PlayerPlugin)
            .add(BlockPlugin)
            .add(WorldGenerationPlugin)
    }
}
