use {
    crate::{
        components::player::*,
        constants::{
            player::*,
            world_generation::{ENTITY_LAYER, TILE_SIZE},
        },
        prelude::*,
        rgb_u8,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub fn spawn_player(mut cmds: Commands) {
    cmds.spawn((
        Player,
        Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
        KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(TILE_SIZE.y),
                min_width: CharacterLength::Absolute(TILE_SIZE.x),
                ..default()
            }),
            snap_to_ground: Some(CharacterLength::Absolute(TILE_SIZE.y)),
            ..default()
        },
        Velocity::default(),
        SpriteBundle {
            sprite: Sprite {
                color: rgb_u8!(255, 0, 255),
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., ENTITY_LAYER),
            ..default()
        },
        InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert_multiple([
                    (MouseButton::Left, Action::Attack),
                    (MouseButton::Right, Action::Interact),
                ])
                .insert_many_to_one([KeyCode::W, KeyCode::Up], Action::MoveUp)
                .insert_many_to_one([KeyCode::S, KeyCode::Down], Action::MoveDown)
                .insert_many_to_one([KeyCode::A, KeyCode::Left], Action::MoveLeft)
                .insert_many_to_one([KeyCode::D, KeyCode::Right], Action::MoveRight)
                .insert(KeyCode::Space, Action::Jump)
                .insert_many_to_one(
                    [KeyCode::Plus, KeyCode::NumpadAdd, KeyCode::Equals],
                    Action::ZoomIn,
                )
                .insert_many_to_one(
                    [KeyCode::Minus, KeyCode::NumpadSubtract, KeyCode::Underline],
                    Action::ZoomOut,
                )
                .build(),
        },
    ));
}

pub fn move_player(
    mut player_qry: Query<&mut Velocity, With<Player>>,
    action_state_qry: Query<&ActionState<Action>, With<Player>>,
) {
    let mut player_vel = player_qry.single_mut();
    let action_state = action_state_qry.single();
    let mvmt_amt = DEFAULT_PLAYER_MOVE_AMOUNT;

    // if action_state.pressed(Action::MoveUp) {
    //     player_vel.linvel.y += mvmt_amt;
    // }
    // if action_state.pressed(Action::MoveDown) {
    //     player_vel.linvel.y -= mvmt_amt;
    // }
    if action_state.pressed(Action::MoveLeft) {
        player_vel.linvel.x -= mvmt_amt;
    }
    if action_state.pressed(Action::MoveRight) {
        player_vel.linvel.x += mvmt_amt;
    }
    if action_state.just_pressed(Action::Jump) && player_vel.linvel.y == 0. {
        player_vel.linvel.y += 200.;
    }
}
