use {
    super::world_generation::ForegroundTilemap, crate::core::game_state::GameState,
    bevy::prelude::*, bevy_ecs_tilemap::prelude::*,
};

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum TileSystem {
    RemoveInvalidTilePositions,
}

pub(super) struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileBreakEvent>().add_system(
            remove_invalid_tile_positions
                .in_set(TileSystem::RemoveInvalidTilePositions)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Tile;

pub struct TileBreakEvent {
    tile_position: TilePos,
}

fn remove_invalid_tile_positions(
    mut tile_break_evr: EventReader<TileBreakEvent>,
    mut fg_tile_storage_qry: Query<&mut TileStorage, With<ForegroundTilemap>>,
) {
    let mut fg_tile_storage = fg_tile_storage_qry.single_mut();

    for TileBreakEvent {
        tile_position: tile_pos,
    } in tile_break_evr.iter()
    {
        fg_tile_storage.remove(tile_pos);
    }
}
