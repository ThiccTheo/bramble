pub mod data;
pub mod funcs;

use {crate::prelude::*, bevy::prelude::*, bevy_ecs_tilemap::prelude::*, data::*, funcs::*};

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
const TILE_MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };
#[allow(dead_code)]
pub const BACKGROUND_LAYER: f32 = 1.;
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
                add_colliders_to_tiles,
            )
                .chain()
                .in_schedule(OnEnter(GameState::Playing)),
        );
    }
}
