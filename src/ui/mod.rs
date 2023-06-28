pub mod hotbar;
pub mod ui_root;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    hotbar::HotbarPlugin,
    ui_root::UiRootPlugin,
};

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(HotbarPlugin)
            .add(UiRootPlugin)
    }
}
