use {
    super::world_generation::{ENTITY_LAYER, TILE_SIZE},
    crate::{
        core::{
            animation::Flippable,
            game_state::GameState,
            mouse_position::MousePosition,
            physics::{BoundingBox, PhysicsSystem},
        },
        logic::{health::DamageEvent, inventory::Inventory},
    },
    bevy::{prelude::*, sprite::collide_aabb},
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub const PLAYER_SIZE: Vec2 = Vec2::new(12., 21.);
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
        app
            .add_systems(
                (
                    load_player_texture,
                    apply_system_buffers,
                    spawn_player.in_set(PlayerSystem::SpawnPlayer),
                )
                    .chain()
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
    DropItem,
    ToggleInventory,
    NextItem,
    PreviousItem,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
struct PlayerTexture(Handle<TextureAtlas>);

fn load_player_texture(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img = assets.load("images/player.png");
    let tex_atlas = TextureAtlas::from_grid(
        img,
        Vec2::new(16., 32.),
        1,
        1,
        Some(Vec2::new(2., 6.)),
        None,
    );
    cmds.insert_resource(PlayerTexture(tex_atlases.add(tex_atlas)));
}

fn spawn_player(mut cmds: Commands, player_texture: Res<PlayerTexture>, assets: Res<AssetServer>) {
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
        Flippable::default(),
        Velocity::default(),
        Friction::new(DEFAULT_PLAYER_FRICTION_COEFFICIENT),
        SpriteSheetBundle {
            transform: Transform::from_xyz(0., 100., ENTITY_LAYER),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: player_texture.0.clone(),
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
                .insert_many_to_one([KeyCode::LAlt, KeyCode::I], PlayerControl::ToggleInventory)
                .insert(KeyCode::Q, PlayerControl::DropItem)
                .build(),
        },
        Inventory {
            keep_items: true,
            items: vec![None, None, None],
            item_slot_count: 3,
        },
    ))
    .insert(Name::new("Player"))
    .with_children(|player| {
        player.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(1., 5., 1.),
                texture: assets.load("images/sclera.png"),
                ..default()
            },
            Flippable::default(),
        ));
        player.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(1., 5., 1.),
                texture: assets.load("images/iris.png"),
                ..default()
            },
            Flippable::default(),
        ));
    });
}

fn move_player(
    mut player_qry: Query<
        (
            &mut Flippable,
            &KinematicCharacterControllerOutput,
            &mut Velocity,
        ),
        With<Player>,
    >,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
) {
    let Ok((mut player_flippable, player_ctrl_out, mut player_vel)) = player_qry.get_single_mut() else {return};
    let action_state = action_state_qry.single();

    if action_state.pressed(PlayerControl::MoveLeft) {
        player_vel.linvel.x -= DEFAULT_PLAYER_MOVE_AMOUNT;
        player_flippable.flip_x = true;
    }
    if action_state.pressed(PlayerControl::MoveRight) {
        player_vel.linvel.x += DEFAULT_PLAYER_MOVE_AMOUNT;
        player_flippable.flip_x = false;
    }
    if action_state.just_pressed(PlayerControl::Jump) && player_ctrl_out.grounded {
        player_vel.linvel.y += DEFAULT_PLAYER_JUMP_POWER;
    }
}

fn attack(
    mut player_qry: Query<(&Transform, &mut Flippable), With<Player>>,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    tiles_qry: Query<(Entity, &Transform)>,
    mut dmg_evw: EventWriter<DamageEvent>,
    mouse_pos: Res<MousePosition>,
) {
    let (player_transform, mut player_flippable) = player_qry.single_mut();
    let action_state = action_state_qry.single();

    if action_state.pressed(PlayerControl::Attack) {
        if mouse_pos.0.x < player_transform.translation.x {
            player_flippable.flip_x = true;
        } else if mouse_pos.0.x > player_transform.translation.x {
            player_flippable.flip_x = false;
        }
        for (tile_id, tile_transform) in tiles_qry.iter() {
            if collide_aabb::collide(
                mouse_pos.0.extend(tile_transform.translation.z),
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
