use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

pub const TILE_SIZE: Vec2 = Vec2::splat(64.);
pub const AVG_TILE_DIM: f32 = (TILE_SIZE.x + TILE_SIZE.y) / 2.;

#[derive(Component)]
pub struct Tile;

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec3,
}

fn tile_spawner(mut tile_spawn_evr: EventReader<TileSpawnEvent>, mut cmds: Commands) {
    for &TileSpawnEvent { pos } in tile_spawn_evr.read() {
        // println!("{pos:?}");
        cmds.spawn((
            Tile,
            StateScoped(GameState::Playing),
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
			RigidBody::Fixed,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::Srgba(Srgba::RED),
                    custom_size: Some(TILE_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
        ));
    }
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>().add_systems(
        Update,
        tile_spawner.run_if(in_state(GameState::Playing).and_then(on_event::<TileSpawnEvent>())),
    );
}
