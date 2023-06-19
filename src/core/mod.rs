pub mod game_state;
pub mod graphics;
pub mod mouse_position;
pub mod physics;

use {
    bevy::{app::PluginGroupBuilder, prelude::*},
    graphics::GraphicsPlugin,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
};

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GraphicsPlugin)
            .add(PhysicsPlugin)
            .add(MousePositionPlugin)
    }
}
