use {
    super::inventory::{Inventory, ItemDropEvent},
    crate::states::game_state::GameState,
    bevy::prelude::*,
};

pub(super) struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>()
            .add_system(on_damage_dealt.in_set(OnUpdate(GameState::Playing)))
            .add_system(remove_dead_entities.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

pub struct DamageDealtEvent {
    pub damage_dealt: i32,
    pub target_id: Entity,
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
        let target_id = *target_id;
        let Ok(mut target_hp) = hp_qry.get_mut(target_id) else { continue };
        target_hp.0 -= *damage_dealt;

        if target_hp.0 <= 0 {
            if let Ok(inventory) = inventory_qry.get_mut(target_id) {
                for (slot_idx, item_slot) in inventory.item_slots.iter().enumerate() {
                    let Some(items) = item_slot else { continue };

                    for _ in items.iter() {
                        item_drop_evw.send(ItemDropEvent {
                            inventory_id: target_id,
                            item_slot: slot_idx,
                        });
                    }
                }
            }
        }
    }
}

fn remove_dead_entities(mut cmds: Commands, hp_qry: Query<(Entity, &Health)>) {
    for id in hp_qry
        .iter()
        .filter(|(_, hp)| !hp.0.is_positive())
        .map(|(id, _)| id)
    {
        cmds.entity(id).despawn_recursive();
    }
}
