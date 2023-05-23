use {
    crate::{prelude::*, systems::physics::*},
    bevy::prelude::*,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (apply_velocity, zero_velocity_on_collision).in_set(OnUpdate(GameState::Playing)),
        );
    }
}
