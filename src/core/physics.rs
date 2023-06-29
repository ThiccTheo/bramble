use {crate::states::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub(super) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1. / 60.))
            .add_system(
                zero_velocity_on_collision
                    .run_if(in_state(GameState::Playing))
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}

#[derive(Component)]
pub struct Gravity(pub f32);

#[derive(Component, Default)]
pub struct TerminalVelocity(pub Velocity);

impl From<Vec2> for TerminalVelocity {
    fn from(value: Vec2) -> Self {
        Self(Velocity::linear(value))
    }
}

#[derive(Component, Clone)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<Vec2> for BoundingBox {
    fn from(value: Vec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}

impl From<BoundingBox> for Vec2 {
    fn from(value: BoundingBox) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

pub fn is_colliding_horizontally(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::X);
    dot_prod > threshold || dot_prod < -threshold
}

pub fn is_colliding_vertically(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::Y);
    dot_prod > threshold || dot_prod < -threshold
}

pub fn zero_velocity_on_collision(
    mut physics_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity), With<Collider>>,
) {
    for (kcc_out, mut vel) in physics_qry.iter_mut() {
        for collision in kcc_out.collisions.iter() {
            let threshold = 0.8;

            if is_colliding_horizontally(collision.toi.normal2, threshold) {
                vel.linvel.x = 0.;
            }
            if is_colliding_vertically(collision.toi.normal2, threshold) {
                vel.linvel.y = 0.;
            }
        }
    }
}
