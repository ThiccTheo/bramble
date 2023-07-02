pub mod combat;
pub mod crafting;
pub mod inventory;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    combat::CombatPlugin,
    crafting::CraftingPlugin,
    inventory::InventoryPlugin,
};

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CombatPlugin)
            .add(CraftingPlugin)
            .add(InventoryPlugin)
    }
}
