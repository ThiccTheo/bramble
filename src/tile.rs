use {
    super::{asset_owner::AssetOwner, game_state::GameState, level::LEVEL_SIZE},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const TILE_SIZE: Vec2 = Vec2::splat(18.);
const TILE_TEXTURE_ATLAS_DIMS: UVec2 = UVec2::new(20, 9);
pub const AVG_TILE_DIM: f32 = (TILE_SIZE.x + TILE_SIZE.y) / 2.;

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
pub struct TileMap([[bool; LEVEL_SIZE.x as usize]; LEVEL_SIZE.y as usize]);

impl Default for TileMap {
    fn default() -> Self {
        Self([[false; LEVEL_SIZE.x as usize]; LEVEL_SIZE.y as usize])
    }
}

#[derive(Component, Clone, Copy)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Event)]
pub struct TileSpawnEvent {
    pub world_pos: Vec3,
    pub tile_pos: TilePosition,
}

fn tile_spawner(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    tile_tex_atlas_layout: Res<AssetOwner<Tile, TextureAtlasLayout>>,
    tile_tex: Res<AssetOwner<Tile, Image>>,
    mut tile_map: ResMut<TileMap>,
) {
    for &TileSpawnEvent {
        world_pos,
        tile_pos,
    } in tile_spawn_evr.read()
    {
        if !tile_map.0[tile_pos.y][tile_pos.x] {
            cmds.spawn((
                Tile,
                StateScoped(GameState::Playing),
                tile_pos,
                Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
                RigidBody::Fixed,
                SpriteBundle {
                    texture: tile_tex.handle(),
                    transform: Transform::from_translation(world_pos),
                    ..default()
                },
                TextureAtlas {
                    layout: tile_tex_atlas_layout.handle(),
                    index: 6 * 20 + 2,
                },
            ));
            tile_map.0[tile_pos.y][tile_pos.x] = true;
        }
    }
}

fn update_tile_textures(
    mut tile_qry: Query<(&TilePosition, &mut TextureAtlas), With<Tile>>,
    tile_map: Res<TileMap>,
) {
    for (&TilePosition { x, y }, mut tile_tex_atlas) in &mut tile_qry {
        let no_up_neighbor = y != 0 && !tile_map.0[y - 1][x];
        let no_down_neighbor = y != LEVEL_SIZE.y as usize - 1 && !tile_map.0[y + 1][x];
        let no_left_neighbor = x != 0 && !tile_map.0[y][x - 1];
        let no_right_neighbor = x != LEVEL_SIZE.x as usize - 1 && !tile_map.0[y][x + 1];

        let mut c_idx = 2;
        let mut r_idx = 6;

        if no_up_neighbor && no_down_neighbor {
            r_idx -= 2;
        } else if no_up_neighbor {
            r_idx -= 1;
        } else if no_down_neighbor {
            r_idx += 1;
        }

        if no_left_neighbor && no_right_neighbor {
            c_idx -= 2;
        } else if no_left_neighbor {
            c_idx -= 1;
        } else if no_right_neighbor {
            c_idx += 1;
        }
        tile_tex_atlas.index = r_idx * TILE_TEXTURE_ATLAS_DIMS.y as usize + c_idx;
    }
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>()
        .add_systems(
            OnEnter(GameState::Setup),
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(AssetOwner::<Tile, Image>::new(
                    asset_server.load("tile.png"),
                ));
                cmds.insert_resource(AssetOwner::<Tile, _>::new(tex_atlas_layouts.add(
                    TextureAtlasLayout::from_grid(
                        UVec2::splat(AVG_TILE_DIM as u32),
                        TILE_TEXTURE_ATLAS_DIMS.x,
                        TILE_TEXTURE_ATLAS_DIMS.y,
                        None,
                        None,
                    ),
                )));
                cmds.insert_resource(TileMap::default());
            },
        )
        .add_systems(
            Update,
            (
                tile_spawner.run_if(on_event::<TileSpawnEvent>()),
                update_tile_textures,
            )
                .run_if(in_state(GameState::Playing)),
        );
}
