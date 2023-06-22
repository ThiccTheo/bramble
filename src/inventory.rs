use {
    crate::{bounding_box::BoundingBox, game_state::GameState, world_generation::ENTITY_LAYER},
    bevy::{prelude::*, sprite::collide_aabb},
    std::time::Duration,
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
                    collect_items,
                    handle_item_pickups.in_set(InventorySystem::HandleItemPickups),
                    update_item_pickup_delays,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component, Default, Reflect)]
pub struct Inventory {
    pub items: Vec<ItemSlot>,
    pub item_slot_count: usize,
    pub keep_items: bool,
}

pub type ItemSlot = Option<Entity>;

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
    assets: Res<AssetServer>,
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

        cmds.entity(*item_id).insert((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(8.)),
                    ..default()
                },
                transform: *transform,
                texture: assets.load("images/player.png"),
                ..default()
            },
            // RigidBody::Dynamic,
            // Collider::cuboid(4., 4.),
            // Velocity::zero(),
            DroppedItem::new(*inventory_id),
            BoundingBox::new(8., 8.),
        ));
        *item = None;
    }
}

fn handle_item_pickups(
    mut cmds: Commands,
    mut item_pickup_evr: EventReader<ItemPickupEvent>,
    mut inventory_qry: Query<&mut Inventory>,
    mut visibility_qry: Query<&mut Visibility>,
) {
    for ItemPickupEvent {
        item_id,
        inventory_id,
    } in item_pickup_evr.iter()
    {
        let Ok(mut inventory) = inventory_qry.get_mut(*inventory_id) else { continue };
        let Some(empty_slot) = inventory.items.iter_mut().position(|item_slot| item_slot.is_none()) else { continue };
        inventory.items[empty_slot] = Some(*item_id);
        cmds.entity(*item_id)
            .remove::<DroppedItem>()
            .remove::<BoundingBox>()
            .remove::<Transform>()
            // .remove::<RigidBody>()
            // .remove::<Collider>()
            // .remove::<Velocity>()
            ;

        let Ok(mut visibility) = visibility_qry.get_mut(*item_id) else { continue };
        *visibility = Visibility::Hidden;
    }
}

fn collect_items(
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
