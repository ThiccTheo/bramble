use {
    crate::{game_state::GameState, physics::BoundingBox, rgb_u8, world_generation::ENTITY_LAYER},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub const PLAYER_SIZE: Vec2 = Vec2::splat(16.);
const DEFAULT_PLAYER_MOVE_AMOUNT: f32 = 20.;
const DEFAULT_PLAYER_JUMP_POWER: f32 = 300.;
const DEFAULT_PLAYER_FRICTION_COEFFICIENT: f32 = 10.;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_player.in_schedule(OnEnter(GameState::Playing)),
            move_player.in_set(OnUpdate(GameState::Playing)),
        ));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    ZoomIn,
    ZoomOut,
    Attack,
    Interact,
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut cmds: Commands) {
    cmds.spawn((
        Player,
        Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
        KinematicCharacterController {
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Absolute(TILE_SIZE.y),
            //     min_width: CharacterLength::Absolute(TILE_SIZE.x),
            //     ..default()
            // }),
            // snap_to_ground: Some(CharacterLength::Absolute(TILE_SIZE.y)),
            ..default()
        },
        Velocity::default(),
        Friction::new(DEFAULT_PLAYER_FRICTION_COEFFICIENT),
        SpriteBundle {
            sprite: Sprite {
                color: rgb_u8!(255, 0, 255),
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(0., 1000., ENTITY_LAYER),
            ..default()
        },
        BoundingBox::from(PLAYER_SIZE),
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert_multiple([
                    (MouseButton::Left, PlayerAction::Attack),
                    (MouseButton::Right, PlayerAction::Interact),
                ])
                .insert_many_to_one([KeyCode::W, KeyCode::Up], PlayerAction::MoveUp)
                .insert_many_to_one([KeyCode::S, KeyCode::Down], PlayerAction::MoveDown)
                .insert_many_to_one([KeyCode::A, KeyCode::Left], PlayerAction::MoveLeft)
                .insert_many_to_one([KeyCode::D, KeyCode::Right], PlayerAction::MoveRight)
                .insert(KeyCode::Space, PlayerAction::Jump)
                .insert_many_to_one(
                    [KeyCode::Plus, KeyCode::NumpadAdd, KeyCode::Equals],
                    PlayerAction::ZoomIn,
                )
                .insert_many_to_one(
                    [KeyCode::Minus, KeyCode::NumpadSubtract, KeyCode::Underline],
                    PlayerAction::ZoomOut,
                )
                .build(),
        },
    ));
}

// Reminder: Try to remove this pub
pub fn move_player(
    mut player_qry: Query<(&KinematicCharacterControllerOutput, &mut Velocity), With<Player>>,
    action_state_qry: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    let Ok((player_ctrl_out, mut player_vel)) = player_qry.get_single_mut() else {return};
    let action_state = action_state_qry.single();

    if action_state.pressed(PlayerAction::MoveLeft) {
        player_vel.linvel.x -= DEFAULT_PLAYER_MOVE_AMOUNT;
    }
    if action_state.pressed(PlayerAction::MoveRight) {
        player_vel.linvel.x += DEFAULT_PLAYER_MOVE_AMOUNT;
    }
    if action_state.just_pressed(PlayerAction::Jump) && player_ctrl_out.grounded {
        player_vel.linvel.y += DEFAULT_PLAYER_JUMP_POWER;
    }
}
