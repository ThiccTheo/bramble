use {
    crate::{
        components::world_generation::*, constants::world_generation::*,
        resources::world_generation::*,
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    noise::{
        utils::{NoiseMapBuilder, PlaneMapBuilder},
        Fbm, Perlin,
    },
};

pub fn spawn_tilemap(mut cmds: Commands, assets: Res<AssetServer>) {
    let src_mod = Fbm::<Perlin>::new(10);
    let perlin_map = PlaneMapBuilder::<_, 2>::new(&src_mod)
        .set_size(TILE_MAP_SIZE.x as usize, TILE_MAP_SIZE.y as usize)
        .build();

    let tilemap_tex = assets.load("images/tiles.png");
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TILE_MAP_SIZE);

    for y in 0..TILE_MAP_SIZE.y {
        for x in 0..TILE_MAP_SIZE.x {
            let tile_pos = TilePos { x, y };

            if perlin_map.get_value(x as usize, y as usize) > 0.5 {
                let tile_id = cmds
                    .spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(0),
                        tilemap_id: TilemapId(tilemap_id),
                        ..default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_id);
            }
        }
    }
    cmds.insert_resource(PerlinMap(perlin_map));

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

pub fn add_colliders_to_tiles(
    mut cmds: Commands,
    fg_tilemap_qry: Query<&TileStorage, With<ForegroundTilemap>>,
    tile_positions: Query<&TilePos>,
) {
    let fg_tile_storage = fg_tilemap_qry.single();

    for tile_id in fg_tile_storage.iter() {
        let Some(tile_id) = tile_id else { continue };
        let Ok(tile_pos) = tile_positions.get(*tile_id) else { continue };
        cmds.entity(*tile_id).insert((
            Transform::from_translation(
                tile_pos
                    .center_in_world(&TILE_SIZE.into(), &TilemapType::Square)
                    .extend(FOREGROUND_LAYER),
            ),
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
        ));
    }
}
