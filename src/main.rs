mod core;
mod logic;
mod ui;
mod world;

use {
    crate::core::{game_state::GameState, graphics::WINDOW_RESOLUTION, CorePlugins},
    bevy::{prelude::*, window::WindowResolution},
    bevy_ecs_tilemap::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
    logic::{inventory::Inventory, LogicPlugins},
    ui::{hotbar::HotbarSlot, UiPlugins},
    world::{player::PlayerControl, WorldPlugins},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bramble"),
                        resolution: WindowResolution::new(WINDOW_RESOLUTION.x, WINDOW_RESOLUTION.y),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_plugin(InputManagerPlugin::<PlayerControl>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::default())
        .register_type::<TextureAtlasSprite>()
        .register_type::<Inventory>()
        .register_type::<HotbarSlot>()
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_plugins(CorePlugins)
        .add_plugins(LogicPlugins)
        .add_plugins(WorldPlugins)
        .add_plugins(UiPlugins)
        .run();
}
