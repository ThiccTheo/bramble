pub mod data;
pub mod funcs;

use {crate::prelude::*, bevy::prelude::*, data::*, funcs::*};

pub const PLAYER_SIZE: Vec2 = Vec2::splat(16.);
const DEFAULT_PLAYER_MOVE_AMOUNT: f32 = 20.;
const DEFAULT_PLAYER_JUMP_POWER: f32 = 300.;
const DEFAULT_PLAYER_FRICTION_COEFFICIENT: f32 = 10.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_player.in_schedule(OnEnter(GameState::Playing)),
            move_player.in_set(OnUpdate(GameState::Playing)),
        ));
    }
}
