use {
    super::{asset_owner::AssetOwner, game_state::GameState, tile::TILE_SIZE},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::{
        control_helpers::{TnuaSimpleAirActionsCounter, TnuaSimpleFallThroughPlatformsHelper},
        prelude::*,
        TnuaGhostSensor, TnuaProximitySensor,
    },
    bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape},
    leafwing_input_manager::prelude::*,
    std::f32::consts::FRAC_PI_4,
};

const PLAYER_SIZE: Vec2 = Vec2::splat(24.);
const PLAYER_COLLIDER_HALF_HEIGHT: f32 = 1.;
const PLAYER_COLLIDER_RADIUS: f32 = 8.;

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub pos: Vec3,
}

#[derive(Actionlike, Reflect, Clone, Hash, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
    Jump,
}

fn player_spawner(
    mut player_spawn_evr: EventReader<PlayerSpawnEvent>,
    mut cmds: Commands,
    player_tex_atlas_layout: Res<AssetOwner<Player, TextureAtlasLayout>>,
    player_tex: Res<AssetOwner<Player, Image>>,
) {
    let &PlayerSpawnEvent { pos } = player_spawn_evr.read().next().unwrap();
    cmds.spawn((
        Player,
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: player_tex.handle(),
            transform: Transform::from_translation(pos.truncate().extend(4.)),
            ..default()
        },
        TextureAtlas {
            layout: player_tex_atlas_layout.handle(),
            index: 8,
        },
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::Left, KeyCode::KeyA),
            (PlayerAction::Right, KeyCode::KeyD),
            (PlayerAction::Up, KeyCode::KeyW),
            (PlayerAction::Down, KeyCode::KeyS),
            (PlayerAction::Jump, KeyCode::Space),
        ])),
        Collider::capsule_y(PLAYER_COLLIDER_HALF_HEIGHT, PLAYER_COLLIDER_RADIUS),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Friction::default(),
        TnuaRapier2dIOBundle::default(),
        TnuaControllerBundle::default(),
        TnuaSimpleAirActionsCounter::default(),
        TnuaSimpleFallThroughPlatformsHelper::default(),
        TnuaGhostSensor::default(),
        TnuaRapier2dSensorShape(Collider::cuboid(PLAYER_COLLIDER_RADIUS - 1., 0.)),
    ));
}

pub fn player_movement(
    mut player_qry: Query<
        (
            &ActionState<PlayerAction>,
            &mut TnuaController,
            &mut TnuaSimpleAirActionsCounter,
            &mut TnuaSimpleFallThroughPlatformsHelper,
            &TnuaGhostSensor,
            &mut TnuaProximitySensor,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    let Ok((
        player_in,
        mut player_kcc,
        mut player_air_actions_ct,
        mut player_ghost_platforms_helper,
        player_ghost_sensor,
        mut player_prox_sensor,
        mut player_sprite,
    )) = player_qry.get_single_mut()
    else {
        return;
    };

    player_kcc.basis(TnuaBuiltinWalk {
        max_slope: FRAC_PI_4,
        spring_dampening: 0.5,
        float_height: PLAYER_COLLIDER_HALF_HEIGHT + PLAYER_COLLIDER_RADIUS + 1.,
        air_acceleration: 5. * TILE_SIZE.x,
        acceleration: 5. * TILE_SIZE.x,
        desired_velocity: 4.
            * TILE_SIZE.x
            * if player_in.pressed(&PlayerAction::Left) && player_in.released(&PlayerAction::Right)
            {
                player_sprite.flip_x = true;
                -Vec3::X
            } else if player_in.pressed(&PlayerAction::Right)
                && player_in.released(&PlayerAction::Left)
            {
                player_sprite.flip_x = false;
                Vec3::X
            } else {
                Vec3::ZERO
            },
        ..default()
    });

    player_air_actions_ct.update(&player_kcc);

    if player_in.pressed(&PlayerAction::Jump) {
        player_kcc.action(TnuaBuiltinJump {
            height: TILE_SIZE.y * 1.5,
            allow_in_air: player_air_actions_ct.air_count_for(TnuaBuiltinJump::NAME) == 0,
            ..default()
        });
    }

    let mut _ghost_platforms_handle = player_ghost_platforms_helper.with(
        &mut player_prox_sensor,
        player_ghost_sensor,
        PLAYER_COLLIDER_HALF_HEIGHT + PLAYER_COLLIDER_RADIUS,
    );

    // if player_in.pressed(&PlayerAction::Down) {
    //     ghost_platforms_handle.try_falling(true);
    // } else if *player_animation_idxs != PlayerAnimation::Jumping.indices() {
    //     ghost_platforms_handle.dont_fall();
    // }
}

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerSpawnEvent>()
        .add_systems(
            OnEnter(GameState::Setup),
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(AssetOwner::<Player, Image>::new(
                    asset_server.load("player.png"),
                ));
                cmds.insert_resource(AssetOwner::<Player, _>::new(tex_atlas_layouts.add(
                    TextureAtlasLayout::from_grid(
                        UVec2::new(PLAYER_SIZE.x as u32, PLAYER_SIZE.y as u32),
                        9,
                        3,
                        None,
                        None,
                    ),
                )));
            },
        )
        .add_systems(
            Update,
            player_spawner
                .run_if(in_state(GameState::Playing).and_then(on_event::<PlayerSpawnEvent>())),
        )
        .add_systems(
            FixedUpdate,
            player_movement
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(GameState::Playing)),
        );
}
