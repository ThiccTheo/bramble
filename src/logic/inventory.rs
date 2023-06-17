use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Inventory {
    pub keep_items: bool,
    pub items: Vec<(Entity, usize)>,
    pub max_num_items: usize,
}

// fn drop_items_on_despawn(mut cmds: Commands, mut inventory_qry: Query<(&mut Inventory, &Transform)>) {
// 	for (mut inventory, transform) in inventory_qry.iter_mut().filter(|(inventory, _)| !inventory.keep_items) {
// 		inventory.items.iter_mut().for_each(|(_, _)| {
// 			cmds.spawn(SpriteBundle {
// 				sprite: Sprite { color: (), flip_x: (), flip_y: (), custom_size: (), rect: (), anchor: () }
// 				transform,
// 				..default()
// 			});
// 		} );
// 	}
// }
