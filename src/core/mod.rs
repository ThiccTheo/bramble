pub mod animation;
pub mod graphics;
pub mod mouse_position;
pub mod physics;

use {
    crate::world::player::PlayerControl,
    animation::AnimationPlugin,
    bevy::{
        app::PluginGroupBuilder,
        prelude::*, /*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}*/
    },
    bevy_rapier2d::prelude::*,
    graphics::GraphicsPlugin,
    leafwing_input_manager::prelude::*,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
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
            .add(PhysicsPlugin)
            .add(MousePositionPlugin)
            .add(AnimationPlugin)
            .add(GraphicsPlugin)
    }
}
