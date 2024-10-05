use {super::game_state::GameState, bevy::prelude::*, bevy_rapier2d::prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub pos: Vec3,
}

fn player_spawner(mut player_spawn_evr: EventReader<PlayerSpawnEvent>, mut cmds: Commands) {
    let &PlayerSpawnEvent { pos } = player_spawn_evr.read().next().unwrap();
    cmds.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::Srgba(Srgba::GREEN),
                custom_size: Some(Vec2::new(20., 40.)),
                ..default()
            },
            transform: Transform::from_translation(pos.truncate().extend(4.)),
            ..default()
        },
    ));
}

pub fn plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>().add_systems(
        Update,
        player_spawner
            .run_if(in_state(GameState::Playing).and_then(on_event::<PlayerSpawnEvent>())),
    );
}
