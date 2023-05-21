use {bevy::prelude::*, leafwing_input_manager::prelude::*};

#[macro_export]
macro_rules! rgb_u8 {
    ($r:expr, $g:expr, $b:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: 255.,
        }
    };

    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: $a as f32 / 255.,
        }
    };
}

#[derive(States, Default, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum GameState {
    #[default]
    Playing,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    ZoomIn,
    ZoomOut,
    Attack,
    Interact,
}
