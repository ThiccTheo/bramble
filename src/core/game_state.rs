use bevy::prelude::*;

#[derive(States, Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Playing,
}
