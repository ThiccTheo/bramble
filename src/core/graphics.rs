use {
    super::{game_state::GameState, mouse_position::MousePosition, physics::BoundingBox},
    bevy::{prelude::*, sprite::collide_aabb},
};

#[macro_export]
macro_rules! rgb_u8 {
    ($r:expr, $g:expr, $b:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: 255.,
        }
    };

    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: $a as f32 / 255.,
        }
    };
}

pub const WINDOW_RESOLUTION: Vec2 = Vec2::new(1280., 720.);
const BACKGROUND_COLOR: Color = rgb_u8!(135, 206, 250);
const OUTLINE_THICKNESS: Val = Val::Px(2.);

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum GraphicsSystem {
    AddOutlinesToHighlightables,
    HighlightTargetOnHover,
}

pub(super) struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(
                (
                    highlight_target_on_hover.in_set(GraphicsSystem::HighlightTargetOnHover),
                    add_outlines_to_highlightables
                        .in_set(GraphicsSystem::AddOutlinesToHighlightables),
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Highlightable;

fn add_outlines_to_highlightables(
    mut cmds: Commands,
    highlightable_qry: Query<(Entity, &BoundingBox), With<Highlightable>>,
) {
    // for (id, hitbox) in highlightable_qry.iter() {
    //     cmds.entity(id).with_children(|parent| {
    //         parent.spawn(SpriteBundle {
    //             sprite: Sprite {
    //                 custom_size: Some(hitbox.clone().into()),
    //                 ..default()
    //             },
    //             ..default()
    //         });
    //     });
    // }
}

fn highlight_target_on_hover(
    interactable_qry: Query<(&BoundingBox, &Transform), With<Highlightable>>,
    mouse_pos: Res<MousePosition>,
) {
    let Some((_interactable_hitbox, _interactable_transform)) = interactable_qry
        .iter()
        .find(|(hitbox, transform)| {
            collide_aabb::collide(
                mouse_pos.0.extend(transform.translation.z),
                Vec2::ONE,
                transform.translation,
                Vec2::new(hitbox.width, hitbox.height),
            )
            .is_some()
        }) else { return };
}
