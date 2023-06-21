pub mod hotbar;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    hotbar::HotbarPlugin,
};

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(HotbarPlugin)
    }
}
