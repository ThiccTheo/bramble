use {
    super::*,
    crate::{graphics::data::Interactable, physics::data::BoundingBox},
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    noise::{
        utils::{NoiseMapBuilder, PlaneMapBuilder},
        Fbm, Perlin,
    },
    rand::Rng,
};

pub(super) fn generate_world_seed(mut cmds: Commands) {
    cmds.insert_resource(WorldSeed(rand::thread_rng().gen_range(u32::MIN..=u32::MAX)));
}

pub(super) fn create_perlin_map(mut cmds: Commands, seed: Res<WorldSeed>) {
    let src_mod = Fbm::<Perlin>::new(seed.0);
    let perlin_map = PlaneMapBuilder::<_, 2>::new(&src_mod)
        .set_size(TILE_MAP_SIZE.x as usize, TILE_MAP_SIZE.y as usize)
        .build();

    cmds.insert_resource(PerlinMap(perlin_map));
}

pub(super) fn spawn_tilemap(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    perlin_map: Res<PerlinMap>,
) {
    let tilemap_tex = assets.load("images/tiles.png");
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TILE_MAP_SIZE);

    for y in 0..TILE_MAP_SIZE.y {
        for x in 0..TILE_MAP_SIZE.x {
            let tile_pos = TilePos { x, y };

            if perlin_map.0.get_value(x as usize, y as usize) > 0.1 {
                let tile_id = cmds
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(0),
                            tilemap_id: TilemapId(tilemap_id),
                            ..default()
                        },
                        Interactable,
                        BoundingBox::new(TILE_SIZE.x, TILE_SIZE.y),
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_id);
            }
        }
    }
    let map_type = TilemapType::default();
    let grid_size = TILE_SIZE.into();

    cmds.entity(tilemap_id).insert((
        TilemapBundle {
            grid_size,
            size: TILE_MAP_SIZE,
            tile_size: TILE_SIZE,
            map_type,
            storage: tile_storage,
            texture: TilemapTexture::Single(tilemap_tex),
            transform: get_tilemap_center_transform(
                &TILE_MAP_SIZE,
                &grid_size,
                &map_type,
                FOREGROUND_LAYER,
            ),
            ..default()
        },
        ForegroundTilemap,
    ));
}

pub(super) fn add_colliders_to_tiles(
    mut cmds: Commands,
    fg_tilemap_qry: Query<(&TileStorage, &Transform), With<ForegroundTilemap>>,
    tile_positions: Query<&TilePos>,
) {
    let (fg_tile_storage, fg_tilemap_transform) = fg_tilemap_qry.single();

    for tile_id in fg_tile_storage.iter().flatten() {
        if let Ok(tile_pos) = tile_positions.get(*tile_id) {
            let Vec2 { x, y } = tile_pos.center_in_world(&TILE_SIZE.into(), &TilemapType::Square);

            cmds.entity(*tile_id).insert((
                TransformBundle::from_transform(
                    *fg_tilemap_transform * Transform::from_xyz(x, y, FOREGROUND_LAYER),
                ),
                Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
            ));
        }
    }
}
