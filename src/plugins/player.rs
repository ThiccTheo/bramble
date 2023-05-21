use {
    crate::{prelude::*, systems::player::*},
    bevy::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_player.in_schedule(OnEnter(GameState::Playing)), move_player));
    }
}
