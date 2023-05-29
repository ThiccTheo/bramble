pub mod data;
pub mod funcs;

#[allow(unused_imports)]
use {
    crate::{prelude::*, rgb_u8},
    bevy::prelude::*,
    data::*,
    funcs::*,
};

const BG_COLOR: Color = rgb_u8!(135, 206, 250);

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_system(highlight_target_on_hover.in_set(OnUpdate(GameState::Playing)));
    }
}
