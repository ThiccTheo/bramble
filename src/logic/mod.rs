use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    health::HealthPlugin,
};

pub mod health;
pub mod inventory;

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HealthPlugin)
    }
}
