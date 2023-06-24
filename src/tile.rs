use {
    super::{
        bounding_box::BoundingBox,
        game_state::GameState,
        health::Health,
        inventory::Inventory,
        item::Item,
        world_generation::{ForegroundTilemap, FOREGROUND_LAYER},
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileBreakEvent>()
            .add_system(add_colliders_to_tiles.in_set(OnUpdate(GameState::Playing)))
            .add_system(on_tile_break.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Tile;

pub struct TileBreakEvent {
    tile_position: TilePos,
}

pub fn spawn_tile(
    cmds: &mut Commands,
    pos: TilePos,
    tex_idx: TileTextureIndex,
    fg_tilemap_id: TilemapId,
    fg_tile_storage: &mut TileStorage,
    assets: &Res<AssetServer>,
) {
    let item_id = cmds
        .spawn((
            Item {
                can_stack: true,
                ..default()
            },
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

    let tile_id = cmds
        .spawn((
            Tile,
            TileBundle {
                position: pos,
                texture_index: tex_idx,
                tilemap_id: fg_tilemap_id,
                ..default()
            },
            Health(100),
            BoundingBox::new(TILE_SIZE.x, TILE_SIZE.y),
            Inventory {
                keep_items: false,
                item_slots: vec![Some(vec![item_id])],
                item_slot_count: 1,
            },
        ))
        .id();

    fg_tile_storage.set(&pos, tile_id);
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

fn add_colliders_to_tiles(
    mut cmds: Commands,
    fg_tilemap_qry: Query<(&TileStorage, &Transform), With<ForegroundTilemap>>,
    tile_pos_qry: Query<&TilePos, (With<Tile>, Without<Collider>)>,
) {
    let (fg_tile_storage, &fg_tilemap_transform) = fg_tilemap_qry.single();

    for &tile_id in fg_tile_storage.iter().flatten() {
        let Ok(tile_pos) = tile_pos_qry.get(tile_id) else { continue };
        let Vec2 { x, y } = tile_pos.center_in_world(&TILE_SIZE.into(), &TilemapType::Square);

        cmds.entity(tile_id).insert((
            TransformBundle::from_transform(
                fg_tilemap_transform * Transform::from_xyz(x, y, FOREGROUND_LAYER),
            ),
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
        ));
    }
}
