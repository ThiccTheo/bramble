use {
    crate::{
        core::physics::BoundingBox,
        logic::{
            combat::Health,
            inventory::{Inventory, InventoryItem},
        },
        states::game_state::GameState,
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const BLOCK_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
pub const BLOCK_LAYER: f32 = 2.;

pub(super) struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileBreakEvent>()
            .add_system(add_colliders_to_blocks.in_set(OnUpdate(GameState::Playing)))
            .add_system(on_tile_break.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct ForegroundTilemap;

pub struct TileBreakEvent {
    tile_position: TilePos,
}

pub fn spawn_block(
    cmds: &mut Commands,
    pos: TilePos,
    tex_idx: TileTextureIndex,
    fg_tilemap_id: TilemapId,
    fg_tile_storage: &mut TileStorage,
    assets: &Res<AssetServer>,
) {
    let item_id = cmds
        .spawn((
            InventoryItem::default(),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(8.)),
                    ..default()
                },
                texture: assets.load("images/player.png"),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .id();

    let block_id = cmds
        .spawn((
            Block,
            TileBundle {
                position: pos,
                texture_index: tex_idx,
                tilemap_id: fg_tilemap_id,
                ..default()
            },
            Health(100),
            BoundingBox::new(BLOCK_SIZE.x, BLOCK_SIZE.y),
            Inventory {
                keep_items: false,
                item_slots: vec![Some(vec![item_id])],
                item_slot_count: 1,
            },
        ))
        .id();

    fg_tile_storage.set(&pos, block_id);
}

fn on_tile_break(
    mut tile_break_evr: EventReader<TileBreakEvent>,
    mut fg_tilemap_qry: Query<&mut TileStorage, With<ForegroundTilemap>>,
) {
    let mut fg_tile_storage = fg_tilemap_qry.single_mut();

    for TileBreakEvent {
        tile_position: tile_pos,
    } in tile_break_evr.iter()
    {
        fg_tile_storage.remove(tile_pos);
    }
}

fn add_colliders_to_blocks(
    mut cmds: Commands,
    fg_tilemap_qry: Query<(&TileStorage, &Transform), With<ForegroundTilemap>>,
    block_qry: Query<&TilePos, (With<Block>, Without<Collider>)>,
) {
    let (fg_tile_storage, &fg_tilemap_transform) = fg_tilemap_qry.single();

    for &tile_id in fg_tile_storage.iter().flatten() {
        let Ok(tile_pos) = block_qry.get(tile_id) else { continue };
        let Vec2 { x, y } = tile_pos.center_in_world(&BLOCK_SIZE.into(), &TilemapType::Square);

        cmds.entity(tile_id).insert((
            TransformBundle::from_transform(
                fg_tilemap_transform * Transform::from_xyz(x, y, BLOCK_LAYER),
            ),
            Collider::cuboid(BLOCK_SIZE.x / 2., BLOCK_SIZE.y / 2.),
        ));
    }
}
