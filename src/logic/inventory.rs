use {
    super::item::{Item, MAX_ITEM_STACK},
    crate::{
        core::physics::BoundingBox, states::game_state::GameState,
        world::world_generation::ENTITY_LAYER,
    },
    bevy::{prelude::*, sprite::collide_aabb},
    std::time::Duration,
};

pub(super) struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemDropEvent>()
            .add_event::<ItemPickupEvent>()
            .add_systems(
                (
                    on_item_drops,
                    intersect_items,
                    on_item_pickups,
                    update_item_pickup_delays,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Inventory {
    pub item_slots: Vec<Option<Vec<Entity>>>,
    pub item_slot_count: usize,
    pub keep_items: bool,
}

#[derive(Component)]
pub struct DroppedItem {
    pub owner: Entity,
    pickup_delay: Timer,
}

impl DroppedItem {
    fn new(owner: Entity) -> Self {
        Self {
            owner,
            pickup_delay: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }

    fn can_be_collected(&self, collector: Entity) -> bool {
        collector != self.owner || self.pickup_delay.finished()
    }
}

pub struct ItemDropEvent {
    pub inventory_id: Entity,
    pub item_slot: usize,
}

pub struct ItemPickupEvent {
    pub item_id: Entity,
    pub inventory_id: Entity,
}

fn on_item_drops(
    mut cmds: Commands,
    mut item_drop_evr: EventReader<ItemDropEvent>,
    mut inventory_qry: Query<(&mut Inventory, &mut Transform)>,
    mut item_qry: Query<(&mut Transform, &mut Visibility), (With<Item>, Without<Inventory>)>,
) {
    for ItemDropEvent {
        inventory_id,
        item_slot,
    } in item_drop_evr.iter()
    {
        let inventory_id = *inventory_id;
        let item_slot = *item_slot;

        let Ok((mut inventory, mut inventory_transform)) = inventory_qry.get_mut(inventory_id) else { continue };
        let Some(Some(items)) = inventory.item_slots.get_mut(item_slot) else { continue };

        inventory_transform.translation.z = ENTITY_LAYER;

        let Some(item_id) = items.pop() else { continue };
        let Ok((mut item_transform, mut item_visibility)) = item_qry.get_mut(item_id) else { continue };

        *item_transform = *inventory_transform;
        *item_visibility = Visibility::Visible;
        cmds.entity(item_id)
            .insert((DroppedItem::new(inventory_id), BoundingBox::new(8., 8.)));

        if inventory.item_slots[item_slot]
            .as_ref()
            .is_some_and(|items| items.is_empty())
        {
            inventory.item_slots[item_slot] = None;
        }
    }
}

fn on_item_pickups(
    mut cmds: Commands,
    mut item_pickup_evr: EventReader<ItemPickupEvent>,
    mut inventory_qry: Query<&mut Inventory>,
    mut visibility_qry: Query<&mut Visibility>,
    item_qry: Query<&Item>,
) {
    for ItemPickupEvent {
        item_id: tmp,
        inventory_id,
    } in item_pickup_evr.iter()
    {
        let Ok(new_item) = item_qry.get(*tmp) else { continue };
        let Ok(mut inventory) = inventory_qry.get_mut(*inventory_id) else { continue };

        let item_slot = inventory.item_slots.iter().position(|item_slot| {
            item_slot.as_ref().is_some_and(|items| {
                items.len() < MAX_ITEM_STACK
                    && items.first().is_some_and(|&item_id| {
                        item_qry
                            .get(item_id)
                            .is_ok_and(|item| item.can_stack && item.id == new_item.id)
                    })
            })
        });

        let item_slot = item_slot.or(inventory
            .item_slots
            .iter()
            .position(|item_slot| item_slot.is_none()));
        let Some(item_slot) = item_slot else { continue };

        if let Some(Some(items)) = inventory.item_slots.get_mut(item_slot) {
            items.push(*tmp);
        } else {
            inventory.item_slots[item_slot] = Some(vec![*tmp]);
        }

        cmds.entity(*tmp).remove::<DroppedItem>();
        let Ok(mut visibility) = visibility_qry.get_mut(*tmp) else { continue };
        *visibility = Visibility::Hidden;
    }
}

fn intersect_items(
    mut item_pickup_evw: EventWriter<ItemPickupEvent>,
    inventory_qry: Query<(Entity, &Transform, &BoundingBox)>,
    dropped_item_qry: Query<(Entity, &Transform, &BoundingBox, &DroppedItem)>,
) {
    for (inventory_id, inventory_transform, inventory_hitbox) in inventory_qry.iter() {
        for (item_id, item_transform, item_hitbox, dropped_item) in dropped_item_qry.iter() {
            if dropped_item.can_be_collected(inventory_id)
                && collide_aabb::collide(
                    inventory_transform.translation,
                    inventory_hitbox.clone().into(),
                    item_transform.translation,
                    item_hitbox.clone().into(),
                )
                .is_some()
            {
                item_pickup_evw.send(ItemPickupEvent {
                    item_id,
                    inventory_id,
                });
            }
        }
    }
}

fn update_item_pickup_delays(time: Res<Time>, mut dropped_item_qry: Query<&mut DroppedItem>) {
    let dt = time.delta();

    for mut item in dropped_item_qry.iter_mut() {
        item.pickup_delay.tick(dt);
    }
}
