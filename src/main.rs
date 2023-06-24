mod bounding_box;
mod combat;
mod crafting;
mod damage;
mod flippable;
mod game_state;
mod health;
mod hotbar;
mod inventory;
mod item;
mod main_camera;
mod mouse_position;
mod physics;
mod player;
mod rgb_u8;
mod tile;
mod ui_root;
mod world_generation;

use {
    bevy::{prelude::*, window::WindowResolution},
    bevy_ecs_tilemap::prelude::*,
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_rapier2d::prelude::*,
    combat::CombatPlugin,
    crafting::CraftingPlugin,
    damage::DamagePlugin,
    flippable::FlippablePlugin,
    game_state::GameState,
    health::HealthPlugin,
    hotbar::HotbarPlugin,
    inventory::InventoryPlugin,
    leafwing_input_manager::prelude::*,
    main_camera::MainCameraPlugin,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerControl, PlayerPlugin},
    tile::TilePlugin,
    ui_root::UiRootPlugin,
    world_generation::WorldGenerationPlugin,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .register_type::<TextureAtlasSprite>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bramble"),
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(InputManagerPlugin::<PlayerControl>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(CraftingPlugin)
        .add_plugin(DamagePlugin)
        .add_plugin(FlippablePlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(HotbarPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(MousePositionPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(UiRootPlugin)
        .add_plugin(WorldGenerationPlugin)
        .run();
}
