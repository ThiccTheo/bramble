use {
    crate::{core::physics::BoundingBox, logic::combat::Health},
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    super::world_generation::BackgroundTilemap,
};

pub const WALL_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
pub const WALL_LAYER: f32 = 1.;

pub(super) struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Wall;

pub fn spawn_wall(
    cmds: &mut Commands,
    pos: TilePos,
    tex_idx: TileTextureIndex,
    bg_tilemap_id: TilemapId,
    bg_tile_storage: &mut TileStorage,
    assets: &Res<AssetServer>,
) {
    let wall_id = cmds
        .spawn((
            Wall,
            TileBundle {
                position: pos,
                texture_index: tex_idx,
                tilemap_id: bg_tilemap_id,
                ..default()
            },
            BoundingBox::new(WALL_SIZE.x, WALL_SIZE.y),
            Health(100),
        ))
        .id();

    bg_tile_storage.set(&pos, wall_id);
}
