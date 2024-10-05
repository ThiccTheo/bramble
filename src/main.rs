mod game_state;
mod level;
mod player;
mod primary_camera;
mod tile;

use {
    bevy::prelude::*, bevy_rapier2d::prelude::*, bevy_tnua::prelude::*,
    bevy_tnua_rapier2d::TnuaRapier2dPlugin, tile::AVG_TILE_DIM,
};

fn main() {
    App::new()
        .insert_resource({
            let mut rapier_cfg = RapierConfiguration::new(AVG_TILE_DIM);
            rapier_cfg.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_cfg
        })
        .add_plugins((
            (
                DefaultPlugins,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(AVG_TILE_DIM)
                    .in_fixed_schedule(),
                //RapierDebugRenderPlugin::default(),
                TnuaRapier2dPlugin::new(FixedUpdate),
                TnuaControllerPlugin::new(FixedUpdate),
            ),
            (
                game_state::plugin,
                primary_camera::plugin,
                level::plugin,
                tile::plugin,
                player::plugin
            ),
        ))
        .run();
}
