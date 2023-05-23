use {
    crate::{prelude::*, systems::world_generation::*},
    bevy::prelude::*,
};

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
