use {crate::constants::physics::*, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub fn apply_velocity(
    mut physics_qry: Query<(&mut KinematicCharacterController, &mut Velocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (mut char_ctrl, mut vel) in physics_qry.iter_mut() {
        // if vel.linvel.y != 0. { 
        // 	vel.linvel.y -= 9.8;
        // }
        vel.linvel.y -= 9.8;
        //vel.linvel.y = f32::max(vel.linvel.y, -DEFAULT_TERMINAL_LINEAR_VELOCITY.y);
        // vel.linvel.x = f32::clamp(
        //     vel.linvel.x,
        //     -DEFAULT_TERMINAL_LINEAR_VELOCITY.x,
        //     DEFAULT_TERMINAL_LINEAR_VELOCITY.x,
        // ); change this
        char_ctrl.translation = Some(vel.linvel * dt);
    }
}

pub fn zero_velocity_on_collision(
    mut physics_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity)>,
) {
    for (char_ctrl_out, mut vel) in physics_qry.iter_mut() {
        if char_ctrl_out.grounded {
            vel.linvel.y = 0.;
        }
        if char_ctrl_out
            .collisions
            .iter()
            .any(|collision| {
            	let dot_prod = collision.toi.normal2.normalize().dot(Vec2::X);
            	let threshold = 0.8;
            	dot_prod > threshold || dot_prod < -threshold
            })
        {
            vel.linvel.x = 0.;
        }
    }
}
