pub mod game_state;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct StatesPlugins;

impl PluginGroup for StatesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
