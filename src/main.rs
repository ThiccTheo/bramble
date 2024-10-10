mod asset_owner;
mod boundary;
mod game_state;
mod level;
mod player;
mod primary_camera;
mod tile;

use {
    bevy::prelude::*, bevy_inspector_egui::quick::WorldInspectorPlugin, bevy_rapier2d::prelude::*,
    bevy_tnua::prelude::*, bevy_tnua_rapier2d::TnuaRapier2dPlugin,
    leafwing_input_manager::prelude::*, player::PlayerAction, static_assertions::const_assert,
    tile::AVG_TILE_DIM,
};

const RESOLUTION: Vec2 = Vec2::new(1280., 720.);
const_assert!(RESOLUTION.x >= 1280. && RESOLUTION.y >= 720.);
const_assert!(RESOLUTION.x / RESOLUTION.y == 16. / 9.);

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
                InputManagerPlugin::<PlayerAction>::default(),
                WorldInspectorPlugin::new(),
            ),
            (
                game_state::game_state_plugin,
                primary_camera::primary_camera_plugin,
                level::level_plugin,
                boundary::boundary_plugin,
                tile::tile_plugin,
                player::player_plugin,
            ),
        ))
        .run();
}
