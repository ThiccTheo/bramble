use bevy::prelude::*;

#[derive(States, Hash, Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameState {
    Setup,
    Playing,
}

pub fn plugin(app: &mut App) {
    app.insert_state(GameState::Setup)
        .enable_state_scoped_entities::<GameState>()
        .add_systems(
            Update,
            (|mut next_state: ResMut<NextState<GameState>>| next_state.set(GameState::Playing))
                .run_if(in_state(GameState::Setup)),
        );
}
