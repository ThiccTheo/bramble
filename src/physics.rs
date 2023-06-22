use {
    super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*, std::time::Duration,
};

const DEFAULT_TERMINAL_VELOCITY: Vec2 = Vec2::new(300., 300.);
const DEFAULT_GRAVITY: f32 = 20.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new(Duration::from_millis(16)))
            .add_systems(
                (
                    zero_velocity_on_collision,
                    apply_velocity,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

pub fn apply_velocity(
    mut physics_qry: Query<(&mut KinematicCharacterController, &mut Velocity, &Friction)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut char_ctrl, mut vel, friction) in physics_qry.iter_mut() {
        vel.linvel.y -= DEFAULT_GRAVITY * dt;

        if vel.linvel.x.is_sign_positive() {
            vel.linvel.x = f32::max((vel.linvel.x - friction.coefficient) * dt, 0.);
        } else if vel.linvel.x.is_sign_negative() {
            vel.linvel.x = f32::min((vel.linvel.x + friction.coefficient) * dt, 0.);
        }
        vel.linvel.x = vel
            .linvel
            .x
            .clamp(-DEFAULT_TERMINAL_VELOCITY.x, DEFAULT_TERMINAL_VELOCITY.x);

        vel.linvel.y = f32::max(vel.linvel.y, -DEFAULT_TERMINAL_VELOCITY.y);
        char_ctrl.translation = Some(vel.linvel);
    }
}

pub fn zero_velocity_on_collision(
    mut physics_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity)>,
) {
    for (char_ctrl_out, mut vel) in physics_qry.iter_mut() {
        for collision in char_ctrl_out.collisions.iter() {
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

fn is_colliding_horizontally(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::X);
    dot_prod > threshold || dot_prod < -threshold
}

fn is_colliding_vertically(normal: Vec2, threshold: f32) -> bool {
    let dot_prod = normal.normalize().dot(Vec2::Y);
    dot_prod > threshold || dot_prod < -threshold
}
