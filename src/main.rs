mod core;
mod logic;
mod states;
mod ui;
mod world;

use {
    crate::core::CorePlugins,
    bevy::{
        prelude::*,
        window::{PresentMode, WindowResolution},
    },
    logic::LogicPlugins,
    states::{game_state::GameState, StatesPlugins},
    ui::UiPlugins,
    world::WorldPlugins,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .register_type::<TextureAtlasSprite>()
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bramble"),
                        resolution: WindowResolution::new(1280., 720.),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(StatesPlugins)
        .add_plugins(CorePlugins)
        .add_plugins(LogicPlugins)
        .add_plugins(UiPlugins)
        .add_plugins(WorldPlugins)
        .run();
}
