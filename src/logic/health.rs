use {
    crate::{
        core::game_state::GameState,
        logic::inventory::{Inventory, InventorySystem, ItemDropEvent},
    },
    bevy::prelude::*,
};

// Subject to change
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum HealthSystem {
    DealDamage,
    RemoveDeadEntities,
}

// Subject to change
pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_systems(
            (
                deal_damage.in_set(HealthSystem::DealDamage),
                remove_dead_entities
                    .in_set(HealthSystem::RemoveDeadEntities)
                    .after(InventorySystem::HandleItemDrops),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Health(pub i32);

pub struct DamageEvent {
    pub damage_dealt: i32,
    pub target_id: Entity,
}

fn deal_damage(
    mut dmg_evr: EventReader<DamageEvent>,
    mut hp_qry: Query<&mut Health>,
    mut inventory_qry: Query<&mut Inventory>,
    mut item_drop_evw: EventWriter<ItemDropEvent>,
) {
    for DamageEvent {
        damage_dealt,
        target_id,
    } in dmg_evr.iter()
    {
        let Ok(mut target_hp) = hp_qry.get_mut(*target_id) else { continue };
        target_hp.0 -= *damage_dealt;

        if target_hp.0 <= 0 {
            if let Ok(inventory) = inventory_qry.get_mut(*target_id) {
                for (slot_idx, &item) in inventory.items.iter().enumerate() {
                    let Some(item_id) = item else { continue };
                    item_drop_evw.send(ItemDropEvent {
                        item_id,
                        inventory_id: *target_id,
                        item_slot: slot_idx,
                    });
                }
            }
        }
    }
}

fn remove_dead_entities(mut cmds: Commands, hp_qry: Query<(Entity, &Health)>) {
    for id in hp_qry.iter().filter(|(_, hp)| hp.0 <= 0).map(|(id, _)| id) {
        cmds.entity(id).despawn_recursive();
    }
}
