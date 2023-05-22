use bevy_ecs_tilemap::prelude::*;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
pub const TILE_MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };
pub const BACKGROUND_LAYER: f32 = 1.;
pub const FOREGROUND_LAYER: f32 = 2.;
pub const ENTITY_LAYER: f32 = 3.;
