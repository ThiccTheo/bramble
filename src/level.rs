use {
    super::{
        game_state::GameState,
        player::PlayerSpawnEvent,
        tile::{TileSpawnEvent, TILE_SIZE},
    },
    crate::RESOLUTION,
    bevy::prelude::*,
    noise::{core::perlin, permutationtable::PermutationTable, utils::PlaneMapBuilder},
    static_assertions::const_assert,
    std::ops::Range,
};

pub const LEVEL_SIZE: Vec2 = Vec2::new(250., 100.);
const_assert!(
    LEVEL_SIZE.x * TILE_SIZE.x >= RESOLUTION.x && LEVEL_SIZE.y * TILE_SIZE.y >= RESOLUTION.y
);

const SKY: Range<usize> = 0..LEVEL_SIZE.y as usize / 3;
const LAND: Range<usize> = SKY.end..LEVEL_SIZE.y as usize;

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

            if LAND.contains(&y) && perlin_map.get_value(x, y) > 0.2 {
                tile_spawn_evw.send(TileSpawnEvent { pos });
            }
            if x == LEVEL_SIZE.x as usize / 2 && y == LAND.start - 1 {
                player_spawn_evw.send(PlayerSpawnEvent { pos });
            }
        }
    }
}

pub fn level_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
}
