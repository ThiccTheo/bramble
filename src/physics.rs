use {
    crate::{player::move_player, game_state::GameState},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

const DEFAULT_TERMINAL_VELOCITY: Vec2 = Vec2::new(100., 300.);
const DEFAULT_GRAVITY: f32 = 9.8;

pub(super) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                zero_velocity_on_collision.before(move_player),
                apply_velocity.after(move_player),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component, Clone)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            width: w,
            height: h,
        }
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

fn apply_velocity(
    mut physics_qry: Query<(&mut KinematicCharacterController, &mut Velocity, &Friction)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut char_ctrl, mut vel, friction) in physics_qry.iter_mut() {
        vel.linvel.y -= DEFAULT_GRAVITY;

        if vel.linvel.x.is_sign_positive() {
            vel.linvel.x = f32::max(vel.linvel.x - friction.coefficient, 0.);
        } else if vel.linvel.x.is_sign_negative() {
            vel.linvel.x = f32::min(vel.linvel.x + friction.coefficient, 0.);
        }
        vel.linvel.x = vel
            .linvel
            .x
            .clamp(-DEFAULT_TERMINAL_VELOCITY.x, DEFAULT_TERMINAL_VELOCITY.x);

        vel.linvel.y = f32::max(vel.linvel.y, -DEFAULT_TERMINAL_VELOCITY.y);
        char_ctrl.translation = Some(vel.linvel * dt);
    }
}

fn zero_velocity_on_collision(
    mut physics_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity)>,
) {
    for (char_ctrl_out, mut vel) in physics_qry.iter_mut() {
        for collision in char_ctrl_out.collisions.iter() {
            let (dot_prod_x, dot_prod_y) = (
                collision.toi.normal2.normalize().dot(Vec2::X),
                collision.toi.normal2.normalize().dot(Vec2::Y),
            );
            let threshold = 0.8;

            if dot_prod_x > threshold || dot_prod_x < -threshold {
                vel.linvel.x = 0.;
            }
            if dot_prod_y > threshold || dot_prod_y < -threshold {
                vel.linvel.y = 0.;
            }
        }
    }
}
