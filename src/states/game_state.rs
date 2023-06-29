use bevy::prelude::*;

pub(super) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}

#[derive(States, Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Playing,
}
