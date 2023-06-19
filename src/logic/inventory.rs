use {
    crate::{core::game_state::GameState, world::world_generation::ENTITY_LAYER},
    bevy::prelude::*,
};

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum InventorySystem {
    HandleItemDrops,
    HandleItemPickups,
}

pub(super) struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemDropEvent>()
            .add_event::<ItemPickupEvent>()
            .add_systems(
                (
                    handle_item_drops.in_set(InventorySystem::HandleItemDrops),
                    handle_item_pickups.in_set(InventorySystem::HandleItemPickups),
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<ItemSlot>,
    pub item_slot_count: usize,
    pub keep_items: bool,
}

pub type ItemSlot = Option<Entity>;

pub struct ItemDropEvent {
    pub item_id: Entity,
    pub inventory_id: Entity,
    pub item_slot: usize,
}

pub struct ItemPickupEvent {
    pub item_id: Entity,
    pub inventory_id: Entity,
}

// TODO: Sprite stuff for the item drops
fn handle_item_drops(
    mut cmds: Commands,
    mut item_drop_evr: EventReader<ItemDropEvent>,
    mut inventory_qry: Query<(&mut Inventory, &mut Transform)>,
) {
    for ItemDropEvent {
        item_id,
        inventory_id,
        item_slot,
    } in item_drop_evr.iter()
    {
        let Ok((mut inventory, mut transform)) = inventory_qry.get_mut(*inventory_id) else { continue };
        let Some(item) = inventory.items.get_mut(*item_slot) else { continue };
        transform.translation.z = ENTITY_LAYER;

        cmds.entity(*item_id).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(8.)),
                ..default()
            },
            transform: *transform,
            ..default()
        });
        *item = None;
    }
}

fn handle_item_pickups(
    mut cmds: Commands,
    mut item_pickup_evr: EventReader<ItemPickupEvent>,
    mut inventory_qry: Query<&mut Inventory>,
) {
    for ItemPickupEvent {
        item_id,
        inventory_id,
    } in item_pickup_evr.iter()
    {
        let Ok(mut inventory) = inventory_qry.get_mut(*inventory_id) else { continue };
        let Some(empty_slot) = inventory.items.iter_mut().position(|item_slot| item_slot.is_none()) else { continue };
        inventory.items[empty_slot] = Some(*item_id);
        cmds.entity(*item_id).remove::<SpriteBundle>();
    }
}
