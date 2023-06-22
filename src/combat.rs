use {
    super::{
        damage::DamageDealtEvent,
        game_state::GameState,
        health::Health,
        inventory::{Inventory, ItemDropEvent},
    },
    bevy::prelude::*,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_damage_dealt.in_set(OnUpdate(GameState::Playing)));
    }
}

fn on_damage_dealt(
    mut dmg_evr: EventReader<DamageDealtEvent>,
    mut hp_qry: Query<&mut Health>,
    mut inventory_qry: Query<&mut Inventory>,
    mut item_drop_evw: EventWriter<ItemDropEvent>,
) {
    for DamageDealtEvent {
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
