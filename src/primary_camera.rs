use {
    super::{game_state::GameState, level::LEVEL_SIZE, player::Player, tile::TILE_SIZE},
    bevy::prelude::*,
};

#[derive(Component)]
pub struct PrimaryCamera;

fn follow_player(
    mut prim_cam_qry: Query<&mut Transform, With<PrimaryCamera>>,
    player_qry: Query<&Transform, (With<Player>, Without<PrimaryCamera>)>,
) {
    let mut prim_cam_xform = prim_cam_qry.single_mut();
    let Ok(player_xform) = player_qry.get_single() else {
        return;
    };

    prim_cam_xform.translation = player_xform
        .translation
        .truncate()
        .extend(prim_cam_xform.translation.z);
}

fn clamp_to_level(
    mut prim_cam_qry: Query<
        (&Camera, &OrthographicProjection, &mut Transform),
        With<PrimaryCamera>,
    >,
) {
    let (prim_cam, prim_cam_proj, mut prim_cam_xform) = prim_cam_qry.single_mut();
    let Some(scaled_vp_size) = prim_cam
        .logical_viewport_size()
        .map(|logical_vp_size| logical_vp_size * prim_cam_proj.scale)
    else {
        return;
    };
    let lvl_size_px = LEVEL_SIZE * TILE_SIZE;

    if lvl_size_px.x >= scaled_vp_size.x {
        let (lvl_left_px, lvl_right_px) = (-lvl_size_px.x / 2., lvl_size_px.x / 2.);
        prim_cam_xform.translation.x = prim_cam_xform.translation.x.clamp(
            lvl_left_px + scaled_vp_size.x / 2.,
            lvl_right_px - scaled_vp_size.x / 2.,
        );
    }
    if lvl_size_px.y >= scaled_vp_size.y {
        let (lvl_top_px, lvl_bottom_px) = (lvl_size_px.y / 2., -lvl_size_px.y / 2.);
        prim_cam_xform.translation.y = prim_cam_xform.translation.y.clamp(
            lvl_bottom_px + scaled_vp_size.y / 2.,
            lvl_top_px - scaled_vp_size.y / 2.,
        );
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb_u8(208, 187, 148)))
        .add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn((PrimaryCamera, Camera2dBundle::default()));
        })
        .add_systems(
            FixedPostUpdate,
            (follow_player, clamp_to_level)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
}
