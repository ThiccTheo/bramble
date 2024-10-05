use {
    super::{
        game_state::GameState,
        player::PlayerSpawnEvent,
        tile::{TileSpawnEvent, TILE_SIZE},
    },
    bevy::prelude::*,
    noise::{core::perlin, permutationtable::PermutationTable, utils::PlaneMapBuilder},
};

pub const LEVEL_SIZE: Vec2 = Vec2::new(100., 10.);

fn spawn_level(
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>,
    mut player_spawn_evw: EventWriter<PlayerSpawnEvent>,
) {
    let hasher = PermutationTable::new(0);
    let perlin_map = PlaneMapBuilder::new_fn(|pt| perlin::perlin_2d(pt.into(), &hasher))
        .set_size(LEVEL_SIZE.x as usize, LEVEL_SIZE.y as usize)
        .build();

    for y in 0..LEVEL_SIZE.y as usize {
        for x in 0..LEVEL_SIZE.x as usize {
            let pos = (Transform::from_translation(
                (-Vec2::new(LEVEL_SIZE.x - 1., LEVEL_SIZE.y - 1.) * TILE_SIZE / 2.).extend(0.),
            ) * Transform::from_translation(
                (Vec2::new(x as f32, LEVEL_SIZE.y - y as f32 - 1.) * TILE_SIZE).extend(0.),
            ))
            .translation;

            if y >= LEVEL_SIZE.y as usize / 3 {
                tile_spawn_evw.send(TileSpawnEvent { pos });
            }
            if y == LEVEL_SIZE.y as usize / 3 - 1 && x == LEVEL_SIZE.x as usize / 2 {
                player_spawn_evw.send(PlayerSpawnEvent { pos });
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
}
