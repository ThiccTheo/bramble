use {
    crate::{
        core::{game_state::GameState, graphics::Highlightable, physics::BoundingBox},
        logic::{health::Health, inventory::Inventory},
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bevy_rapier2d::prelude::*,
    noise::{
        utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
        Fbm, Perlin,
    },
    rand::Rng,
};

//  (0, 1), (1, 1)
//  (0, 0), (1, 0)

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
const TILE_MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };
pub const BACKGROUND_LAYER: f32 = 1.;
pub const FOREGROUND_LAYER: f32 = 2.;
pub const ENTITY_LAYER: f32 = 3.;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum WorldGenerationSystem {
    GenerateWorldSeed,
    CreatePerlinMap,
    SpawnTilemap,
    AddCollidersToTiles,
}

pub(super) struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                generate_world_seed.in_set(WorldGenerationSystem::GenerateWorldSeed),
                apply_system_buffers,
                create_perlin_map.in_set(WorldGenerationSystem::CreatePerlinMap),
                apply_system_buffers,
                spawn_tilemap.in_set(WorldGenerationSystem::SpawnTilemap),
                apply_system_buffers,
                add_colliders_to_tiles.in_set(WorldGenerationSystem::AddCollidersToTiles),
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

fn spawn_tilemap(mut cmds: Commands, assets: Res<AssetServer>, perlin_map: Res<PerlinMap>) {
    let tilemap_tex = assets.load("images/tile.png");
    let tilemap_id = cmds.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(TILE_MAP_SIZE);

    for y in 0..TILE_MAP_SIZE.y {
        for x in 0..TILE_MAP_SIZE.x {
            let tile_pos = TilePos { x, y };

            if perlin_map
                .0
                .get_value(x as usize, TILE_MAP_SIZE.y as usize - y as usize - 1)
                > 0.1
            {
                // REMOVE THIS
                let tmp_id = cmds.spawn_empty().id();
                let tile_id = cmds
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(if y == TILE_MAP_SIZE.y - 1 {
                                0
                            } else if y > TILE_MAP_SIZE.y / 2 {
                                1
                            } else {
                                2
                            }),
                            tilemap_id: TilemapId(tilemap_id),
                            ..default()
                        },
                        Health(100),
                        Highlightable,
                        BoundingBox::new(TILE_SIZE.x, TILE_SIZE.y),
                        Inventory {
                            keep_items: false,
                            items: vec![Some(tmp_id)],
                            item_slot_count: 1,
                        },
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

fn add_colliders_to_tiles(
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
