mod game_state;
mod graphics;
mod main_camera;
mod physics;
mod player;
mod world_generation;

use {
    bevy::{
        ecs::schedule::{LogLevel, ScheduleBuildSettings},
        prelude::*,
    },
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    graphics::GraphicsPlugin,
    leafwing_input_manager::prelude::*,
    main_camera::MainCameraPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    world_generation::WorldGenerationPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Bramble"),
                ..default()
            }),
            ..default()
        }))
        .edit_schedule(CoreSchedule::Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Ignore,
                ..default()
            });
        })
        .add_state::<GameState>()
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(WorldGenerationPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
