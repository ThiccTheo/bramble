mod components;
mod constants;
mod events;
mod plugins;
mod prelude;
mod resources;
mod systems;

use {
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
    plugins::{main_camera::MainCameraPlugin, player::PlayerPlugin},
    prelude::*,
};

use systems::tilemap::spawn_tilemap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Bramble"),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(spawn_tilemap.in_schedule(OnEnter(GameState::Playing)))
        .run();
}
