use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1. / 60.))
            .add_system(
                zero_velocity_on_collision
                    .run_if(in_state(GameState::Playing))
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
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
