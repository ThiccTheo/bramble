mod core;
mod logic;
mod world;

use {
    crate::core::{game_state::GameState, graphics::WINDOW_RESOLUTION, CorePlugins},
    bevy::{
        ecs::schedule::{LogLevel, ScheduleBuildSettings},
        prelude::*,
        window::WindowResolution,
    },
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
    logic::LogicPlugins,
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
                .set(ImagePlugin::default_linear()),
        )
        .edit_schedule(CoreSchedule::Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Ignore,
                ..default()
            });
        })
        .add_state::<GameState>()
        .add_plugin(InputManagerPlugin::<PlayerControl>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_plugins(CorePlugins)
        .add_plugins(LogicPlugins)
        .add_plugins(WorldPlugins)
        .run();
}
