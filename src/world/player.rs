use {
    super::{block::BLOCK_SIZE, world_generation::ENTITY_LAYER},
    crate::{
        core::{
            animation::Flippable,
            mouse_position::MousePosition,
            physics::{self, BoundingBox, Gravity, TerminalVelocity},
        },
        logic::{
            combat::DamageDealtEvent,
            inventory::{Inventory, ItemDropEvent},
        },
        states::game_state::GameState,
    },
    bevy::{
        input::mouse::{MouseScrollUnit, MouseWheel},
        prelude::*,
        sprite::collide_aabb,
    },
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

pub const PLAYER_SIZE: Vec2 = Vec2::new(12., 21.);

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (load_player_texture, apply_system_buffers, spawn_player)
                .chain()
                .in_schedule(OnEnter(GameState::Playing)),
        )
        .add_systems(
            (
                attack,
                interact,
                drop_item,
                update_current_hotbar_index,
                hotbar_scrolling,
            )
                .in_set(OnUpdate(GameState::Playing)),
        )
        .add_system(
            move_player
                .after(physics::zero_velocity_on_collision)
                .run_if(in_state(GameState::Playing))
                .in_schedule(CoreSchedule::FixedUpdate),
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
    HotbarSlot1,
    HotbarSlot2,
    HotbarSlot3,
    HotbarSlot4,
    HotbarSlot5,
    HotbarSlot6,
    HotbarSlot7,
    HotbarSlot8,
    HotbarSlot9,
    HotbarSlot10,
}

#[derive(Component, Default)]
pub struct Player {
    pub current_hotbar_index: usize,
}

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
        Player::default(),
        Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
        KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(BLOCK_SIZE.y + 1.),
                min_width: CharacterLength::Absolute(BLOCK_SIZE.x - 1.),
                ..default()
            }),
            snap_to_ground: Some(CharacterLength::Absolute(BLOCK_SIZE.y)),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
        Flippable::default(),
        SpriteSheetBundle {
            transform: Transform::from_xyz(0., 100., ENTITY_LAYER),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: player_texture.0.clone(),
            ..default()
        },
        Velocity::default(),
        TerminalVelocity::from(Vec2::splat(100.)),
        Friction::new(100.),
        Gravity(100.),
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
                .insert_many_to_one(
                    [KeyCode::Key1, KeyCode::Numpad1],
                    PlayerControl::HotbarSlot1,
                )
                .insert_many_to_one(
                    [KeyCode::Key2, KeyCode::Numpad2],
                    PlayerControl::HotbarSlot2,
                )
                .insert_many_to_one(
                    [KeyCode::Key3, KeyCode::Numpad3],
                    PlayerControl::HotbarSlot3,
                )
                .insert_many_to_one(
                    [KeyCode::Key4, KeyCode::Numpad4],
                    PlayerControl::HotbarSlot4,
                )
                .insert_many_to_one(
                    [KeyCode::Key5, KeyCode::Numpad5],
                    PlayerControl::HotbarSlot5,
                )
                .insert_many_to_one(
                    [KeyCode::Key6, KeyCode::Numpad6],
                    PlayerControl::HotbarSlot6,
                )
                .insert_many_to_one(
                    [KeyCode::Key7, KeyCode::Numpad7],
                    PlayerControl::HotbarSlot7,
                )
                .insert_many_to_one(
                    [KeyCode::Key8, KeyCode::Numpad8],
                    PlayerControl::HotbarSlot8,
                )
                .insert_many_to_one(
                    [KeyCode::Key9, KeyCode::Numpad9],
                    PlayerControl::HotbarSlot9,
                )
                .insert_many_to_one(
                    [KeyCode::Key0, KeyCode::Numpad0],
                    PlayerControl::HotbarSlot10,
                )
                .build(),
        },
        Inventory {
            keep_items: true,
            item_slots: vec![None; 10],
            item_slot_count: 10,
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

pub fn move_player(
    mut player_qry: Query<
        (
            &ActionState<PlayerControl>,
            &mut Flippable,
            &mut KinematicCharacterController,
            &mut Velocity,
            &TerminalVelocity,
            &Friction,
            &Gravity,
        ),
        With<Player>,
    >,
    ts: Res<FixedTime>,
) {
    let (
        action_state,
        mut player_flippable,
        mut player_kcc,
        mut player_vel,
        player_terminal_vel,
        player_friction,
        player_gravity,
    ) = player_qry.single_mut();

    let dt = ts.period.as_secs_f32();

    let acceleration = 110.;
    let jump_power = 30.;

    if action_state.pressed(PlayerControl::MoveLeft) {
        player_vel.linvel.x -= acceleration * dt;
        player_flippable.flip_x = true;
    }
    if action_state.pressed(PlayerControl::MoveRight) {
        player_vel.linvel.x += acceleration * dt;
        player_flippable.flip_x = false;
    }
    if action_state.just_pressed(PlayerControl::Jump) && player_vel.linvel.y == 0. {
        player_vel.linvel.y = jump_power;
    }

    if player_vel.linvel.x.is_sign_positive() {
        player_vel.linvel.x = f32::max(player_vel.linvel.x - player_friction.coefficient * dt, 0.);
    } else if player_vel.linvel.x.is_sign_negative() {
        player_vel.linvel.x = f32::min(player_vel.linvel.x + player_friction.coefficient * dt, 0.);
    }

    player_vel.linvel.x = player_vel.linvel.x.clamp(
        -player_terminal_vel.0.linvel.x,
        player_terminal_vel.0.linvel.x,
    );

    player_vel.linvel.y = f32::max(
        player_vel.linvel.y - player_gravity.0 * dt,
        -player_terminal_vel.0.linvel.y,
    );

    player_kcc.translation = Some(player_vel.linvel);
}

fn attack(
    mut player_qry: Query<(&Transform, &mut Flippable), With<Player>>,
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    tiles_qry: Query<(Entity, &Transform)>,
    mut dmg_evw: EventWriter<DamageDealtEvent>,
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
                BLOCK_SIZE.into(),
            )
            .is_some()
            {
                dmg_evw.send(DamageDealtEvent {
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

fn update_current_hotbar_index(
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    mut player_qry: Query<&mut Player>,
) {
    let action_state = action_state_qry.single();
    let mut player = player_qry.single_mut();

    player.current_hotbar_index = if action_state.just_pressed(PlayerControl::HotbarSlot1) {
        0
    } else if action_state.just_pressed(PlayerControl::HotbarSlot2) {
        1
    } else if action_state.just_pressed(PlayerControl::HotbarSlot3) {
        2
    } else if action_state.just_pressed(PlayerControl::HotbarSlot4) {
        3
    } else if action_state.just_pressed(PlayerControl::HotbarSlot5) {
        4
    } else if action_state.just_pressed(PlayerControl::HotbarSlot6) {
        5
    } else if action_state.just_pressed(PlayerControl::HotbarSlot7) {
        6
    } else if action_state.just_pressed(PlayerControl::HotbarSlot8) {
        7
    } else if action_state.just_pressed(PlayerControl::HotbarSlot9) {
        8
    } else if action_state.just_pressed(PlayerControl::HotbarSlot10) {
        9
    } else {
        player.current_hotbar_index
    }
}

fn drop_item(
    action_state_qry: Query<&ActionState<PlayerControl>, With<Player>>,
    player_qry: Query<(Entity, &Inventory, &Player)>,
    mut item_drop_evr: EventWriter<ItemDropEvent>,
) {
    let action_state = action_state_qry.single();
    let (player_id, player_inventory, player) = player_qry.single();

    if action_state.just_pressed(PlayerControl::DropItem) {
        let Some(Some(items)) = player_inventory.item_slots.get(player.current_hotbar_index) else { return };

        for _ in items.iter() {
            item_drop_evr.send(ItemDropEvent {
                inventory_id: player_id,
                item_slot: player.current_hotbar_index,
            });
        }
    }
}

fn hotbar_scrolling(mut scroll_evr: EventReader<MouseWheel>, mut player_qry: Query<&mut Player>) {
    let mut player = player_qry.single_mut();

    for e in scroll_evr.iter() {
        if e.unit == MouseScrollUnit::Line {
            let offset = -(e.y.round() as i32);
            let old_idx = player.current_hotbar_index as i32;
            let new_idx = (old_idx + offset).rem_euclid(10);
            player.current_hotbar_index = new_idx as usize;
        }
    }
}
