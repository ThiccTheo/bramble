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
    states::StatesPlugins,
    ui::UiPlugins,
    world::WorldPlugins,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bramble"),
                        resolution: WindowResolution::new(1280., 720.),
                        present_mode: PresentMode::AutoVsync,
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
