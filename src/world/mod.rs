pub mod main_camera;
pub mod player;
pub mod tile;
pub mod world_generation;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    bevy_ecs_tilemap::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    main_camera::MainCameraPlugin,
    player::PlayerPlugin,
    tile::TilePlugin,
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
            .add(TilePlugin)
            .add(WorldGenerationPlugin)
    }
}
