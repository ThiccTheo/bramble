use {bevy::prelude::*, bevy_rapier2d::prelude::*};

#[derive(Component, Default)]
pub struct TerminalVelocity(pub Velocity);

impl From<Vec2> for TerminalVelocity {
    fn from(value: Vec2) -> Self {
        Self(Velocity::linear(value))
    }
}
