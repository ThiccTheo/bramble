use {
    crate::{prelude::*, systems::world_generation::*},
    bevy::prelude::*,
};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (spawn_tilemap, add_colliders_to_tiles)
                .chain()
                .in_schedule(OnEnter(GameState::Playing)),
        );
    }
}
