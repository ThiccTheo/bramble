use {
    crate::constants::tilemap::*,
    bevy::prelude::*,
    bevy_ecs_tilemap::{helpers::filling, prelude::*},
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

    //filling::fill_tilemap(TileTextureIndex(0), TILE_MAP_SIZE, TilemapId(tilemap_id), &mut cmds, &mut tile_storage);

    let map_type = TilemapType::default();
    let grid_size = TILE_SIZE.into();

    cmds.entity(tilemap_id).insert(TilemapBundle {
        grid_size,
        size: TILE_MAP_SIZE,
        tile_size: TILE_SIZE,
        map_type,
        texture: TilemapTexture::Single(tilemap_tex),
        transform: get_tilemap_center_transform(
            &TILE_MAP_SIZE,
            &grid_size,
            &map_type,
            FOREGROUND_LAYER,
        ),
        ..default()
    });
}
