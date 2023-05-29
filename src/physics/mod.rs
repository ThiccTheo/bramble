pub mod data;
pub mod funcs;

#[allow(unused_imports)]
use {
    crate::{player::funcs::move_player, prelude::*},
    bevy::prelude::*,
    data::*,
    funcs::*,
};

const DEFAULT_TERMINAL_VELOCITY: Vec2 = Vec2::new(100., 300.);
const DEFAULT_GRAVITY: f32 = 9.8;

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
