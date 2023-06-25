use {
    super::{
        game_state::GameState,
        tile::{self, TILE_SIZE},
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    noise::{
        utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
        Fbm, Perlin,
    },
    rand::Rng,
};

//  (0, 1), (1, 1)
//  (0, 0), (1, 0)

const TILE_MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };
pub const _BACKGROUND_LAYER: f32 = 1.;
pub const FOREGROUND_LAYER: f32 = 2.;
pub const ENTITY_LAYER: f32 = 3.;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                generate_world_seed,
                apply_system_buffers,
                create_perlin_map,
                apply_system_buffers,
                spawn_tilemap,
                apply_system_buffers,
            )
                .chain()
                .in_schedule(OnEnter(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct BackgroundTilemap;

#[derive(Component)]
pub struct ForegroundTilemap;

#[derive(Resource)]
pub struct PerlinMap(pub NoiseMap);

#[derive(Resource)]
pub struct WorldSeed(pub u32);

fn generate_world_seed(mut cmds: Commands) {
    cmds.insert_resource(WorldSeed(rand::thread_rng().gen_range(u32::MIN..=u32::MAX)));
}

fn create_perlin_map(mut cmds: Commands, seed: Res<WorldSeed>) {
    let src_mod = Fbm::<Perlin>::new(seed.0);
    let perlin_map = PlaneMapBuilder::<_, 2>::new(&src_mod)
        .set_size(TILE_MAP_SIZE.x as usize, TILE_MAP_SIZE.y as usize)
        .set_x_bounds(-5., 5.)
        .set_y_bounds(-5., 5.)
        .build();

    cmds.insert_resource(PerlinMap(perlin_map));
}

pub fn spawn_tilemap(mut cmds: Commands, assets: Res<AssetServer>, perlin_map: Res<PerlinMap>) {
    let tilemap_tex = assets.load("images/tile.png");
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TILE_MAP_SIZE);

    for y in 0..TILE_MAP_SIZE.y {
        for x in 0..TILE_MAP_SIZE.x {
            if perlin_map
                .0
                .get_value(x as usize, TILE_MAP_SIZE.y as usize - y as usize - 1)
                > 0.1
            {
                let tex_idx = TileTextureIndex(if y == TILE_MAP_SIZE.y - 1 {
                    0
                } else if y > TILE_MAP_SIZE.y / 2 {
                    1
                } else {
                    2
                });

                tile::spawn_tile(
                    &mut cmds,
                    TilePos { x, y },
                    tex_idx,
                    TilemapId(tilemap_id),
                    &mut tile_storage,
                    &assets,
                );
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
