mod components;
mod constants;
mod events;
mod plugins;
mod prelude;
mod resources;
mod systems;

use {
    bevy::prelude::*,
    leafwing_input_manager::prelude::*,
    plugins::{main_camera::MainCameraPlugin, player::PlayerPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(MainCameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
