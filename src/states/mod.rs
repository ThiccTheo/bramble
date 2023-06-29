pub mod game_state;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    game_state::GameStatePlugin,
};

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GameStatePlugin)
    }
}
