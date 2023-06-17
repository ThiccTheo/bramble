use {
    super::{
        main_camera::MainCamera,
        world_generation::{ENTITY_LAYER, TILE_SIZE},
    },
    crate::{
        core::{
            game_state::GameState,
            physics::{BoundingBox, PhysicsSystem},
        },
        logic::health::DamageEvent,
    },
    bevy::{prelude::*, sprite::collide_aabb, window::PrimaryWindow},
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub const PLAYER_SIZE: Vec2 = Vec2::new(14., 30.);
const DEFAULT_PLAYER_MOVE_AMOUNT: f32 = 20.;
const DEFAULT_PLAYER_JUMP_POWER: f32 = 300.;
const DEFAULT_PLAYER_FRICTION_COEFFICIENT: f32 = 10.;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum PlayerSystem {
    SpawnPlayer,
    MovePlayer,
    Attack,
    Interact,
}

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_player
                .in_set(PlayerSystem::SpawnPlayer)
                .in_schedule(OnEnter(GameState::Playing)),
        )
        .add_systems(
            (
                move_player
                    .in_set(PlayerSystem::MovePlayer)
                    .after(PhysicsSystem::ZeroVelocityOnCollision)
                    .before(PhysicsSystem::ApplyVelocity),
                attack.in_set(PlayerSystem::Attack),
                interact.in_set(PlayerSystem::Interact),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerControl {
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

fn spawn_player(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        Player,
        Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
        KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(TILE_SIZE.y + 1.),
                min_width: CharacterLength::Absolute(TILE_SIZE.x - 1.),
                ..default()
            }),
            snap_to_ground: Some(CharacterLength::Absolute(TILE_SIZE.y)),
            ..default()
        },
        Velocity::default(),
        Friction::new(DEFAULT_PLAYER_FRICTION_COEFFICIENT),
        SpriteBundle {
            // sprite: Sprite {
            //     custom_size: Some(PLAYER_SIZE),
            //     ..default()
            // },
            transform: Transform::from_xyz(0., 1000., ENTITY_LAYER),
            texture: assets.load("images/player.png"),
            ..default()
        },
        BoundingBox::from(PLAYER_SIZE),
        InputManagerBundle::<PlayerControl> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert_multiple([
                    (MouseButton::Left, PlayerControl::Attack),
                    (MouseButton::Right, PlayerControl::Interact),
                ])
                .insert_many_to_one([KeyCode::W, KeyCode::Up], PlayerControl::MoveUp)
                .insert_many_to_one([KeyCode::S, KeyCode::Down], PlayerControl::MoveDown)
                .insert_many_to_one([KeyCode::A, KeyCode::Left], PlayerControl::MoveLeft)
                .insert_many_to_one([KeyCode::D, KeyCode::Right], PlayerControl::MoveRight)
                .insert(KeyCode::Space, PlayerControl::Jump)
                .insert_many_to_one(
                    [KeyCode::Plus, KeyCode::NumpadAdd, KeyCode::Equals],
                    PlayerControl::ZoomIn,
                )
                .insert_many_to_one(
                    [KeyCode::Minus, KeyCode::NumpadSubtract, KeyCode::Underline],
                    PlayerControl::ZoomOut,
                )
                .build(),
        },
    ));
}

fn move_player(
    mut player_qry: Query<
        (
            &mut Sprite,
            &KinematicCharacterControllerOutput,
            &mut Velocity,
        ),
        With<Player>,
    >,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
) {
    let Ok((mut player_sprite, player_ctrl_out, mut player_vel)) = player_qry.get_single_mut() else {return};
    let action_state = action_state_qry.single();

    if action_state.pressed(PlayerControl::MoveLeft) {
        player_vel.linvel.x -= DEFAULT_PLAYER_MOVE_AMOUNT;
        player_sprite.flip_x = true;
    }
    if action_state.pressed(PlayerControl::MoveRight) {
        player_vel.linvel.x += DEFAULT_PLAYER_MOVE_AMOUNT;
        player_sprite.flip_x = false;
    }
    if action_state.just_pressed(PlayerControl::Jump) && player_ctrl_out.grounded {
        player_vel.linvel.y += DEFAULT_PLAYER_JUMP_POWER;
    }
}

fn attack(
    mut player_qry: Query<(&Transform, &mut Sprite), With<Player>>,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    cam_qry: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    win_qry: Query<&Window, With<PrimaryWindow>>,
    tiles_qry: Query<(Entity, &Transform)>,
    mut dmg_evw: EventWriter<DamageEvent>,
) {
    let (player_transform, mut player_sprite) = player_qry.single_mut();
    let action_state = action_state_qry.single();
    let (cam, cam_transform) = cam_qry.single();
    let win = win_qry.single();
    let Some(mouse_pos) = win.cursor_position().and_then(|pos| cam.viewport_to_world_2d(cam_transform, pos)) else { return };

    if action_state.pressed(PlayerControl::Attack) {
        if mouse_pos.x < player_transform.translation.x {
            player_sprite.flip_x = true;
        } else if mouse_pos.x > player_transform.translation.x {
            player_sprite.flip_x = false;
        }
        for (tile_id, tile_transform) in tiles_qry.iter() {
            if collide_aabb::collide(
                mouse_pos.extend(tile_transform.translation.z),
                Vec2::ONE,
                tile_transform.translation,
                TILE_SIZE.into(),
            )
            .is_some()
            {
                dmg_evw.send(DamageEvent {
                    damage_dealt: 10,
                    target_id: tile_id,
                });
                break;
            }
        }
    }
}

fn interact(action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>) {
    let action_state = action_state_qry.single();

    if action_state.pressed(PlayerControl::Interact) {}
}
