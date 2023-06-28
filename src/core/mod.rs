pub mod bounding_box;
pub mod collisions;
pub mod flippable;
pub mod forces;
pub mod gravity;
pub mod mouse_position;
pub mod rgb_u8;
pub mod terminal_velocity;

use {
    crate::world::player::PlayerControl,
    bevy::{
        app::PluginGroupBuilder,
        prelude::*, /*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}*/
    },
    bevy_rapier2d::prelude::*,
    collisions::CollisionsPlugin,
    forces::ForcesPlugin,
    leafwing_input_manager::prelude::*,
    mouse_position::MousePositionPlugin,
};

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add_plugin(LogDiagnosticsPlugin::default())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add(InputManagerPlugin::<PlayerControl>::default())
            .add(RapierPhysicsPlugin::<NoUserData>::default())
            .add(RapierDebugRenderPlugin::default())
            .add(CollisionsPlugin)
            .add(ForcesPlugin)
            .add(MousePositionPlugin)
    }
}
