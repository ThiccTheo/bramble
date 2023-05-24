use crate::systems::player::move_player;

use {
    crate::{prelude::*, systems::physics::*},
    bevy::prelude::*,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                zero_velocity_on_collision.before(move_player),
                apply_velocity.after(move_player),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}
