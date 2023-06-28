pub mod combat;
pub mod crafting;
pub mod damage;
pub mod health;
pub mod inventory;
pub mod item;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    combat::CombatPlugin,
    crafting::CraftingPlugin,
    damage::DamagePlugin,
    health::HealthPlugin,
    inventory::InventoryPlugin,
};

pub struct LogicPlugins;

impl PluginGroup for LogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CombatPlugin)
            .add(CraftingPlugin)
            .add(DamagePlugin)
            .add(HealthPlugin)
            .add(InventoryPlugin)
    }
}
