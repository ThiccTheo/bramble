use {
    super::{game_state::GameState, level::LEVEL_SIZE, tile::TILE_SIZE},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

#[derive(Component)]
pub struct Boundary;

fn spawn_boundaries(mut cmds: Commands) {
    let lvl_size_px = LEVEL_SIZE * TILE_SIZE;

    cmds.spawn_batch([
        (
            Boundary,
            Transform::from_xyz(-lvl_size_px.x / 2., 0., 0.),
            GlobalTransform::default(),
            Collider::cuboid(0., lvl_size_px.y / 2.),
        ),
        (
            Boundary,
            Transform::from_xyz(lvl_size_px.x / 2., 0., 0.),
            GlobalTransform::default(),
            Collider::cuboid(0., lvl_size_px.y / 2.),
        ),
        (
            Boundary,
            Transform::from_xyz(0., lvl_size_px.y / 2., 0.),
            GlobalTransform::default(),
            Collider::cuboid(lvl_size_px.x / 2., 0.),
        ),
        (
            Boundary,
            Transform::from_xyz(0., -lvl_size_px.y / 2., 0.),
            GlobalTransform::default(),
            Collider::cuboid(lvl_size_px.x / 2., 0.),
        ),
    ]);
}

pub fn boundary_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_boundaries);
}
