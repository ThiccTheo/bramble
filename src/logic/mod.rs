pub mod health;
pub mod inventory;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    health::HealthPlugin,
};

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HealthPlugin)
    }
}
