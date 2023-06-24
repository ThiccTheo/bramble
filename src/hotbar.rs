use {
    crate::{game_state::GameState, inventory::Inventory, player::Player, rgb_u8},
    bevy::prelude::*,
};

const HOTBAR_ITEM_SIZE: Vec2 = Vec2::splat(32.);
const HOTBAR_ITEM_COUNT: usize = 10;
const HOTBAR_ITEM_PADDING: f32 = 5.;
const HOTBAR_WIDTH: f32 =
    HOTBAR_ITEM_SIZE.x * HOTBAR_ITEM_COUNT as f32 + HOTBAR_ITEM_COUNT as f32 * HOTBAR_ITEM_PADDING;
const HOTBAR_HEIGHT: f32 = HOTBAR_ITEM_SIZE.y + 2. * HOTBAR_ITEM_PADDING;
const HOTBAR_SIZE: Size = Size {
    width: Val::Px(HOTBAR_WIDTH),
    height: Val::Px(HOTBAR_HEIGHT),
};

pub(super) struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hotbar.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    populate_hotbar,
                    update_hotbar_items,
                    highlight_selected_slot,
                    click_hotbar_slot,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Hotbar;

#[derive(Component, Default, Reflect)]
pub struct HotbarSlot {
    pub item_id: Option<Entity>,
}

#[derive(Component)]
pub struct HotbarButton(pub usize);

fn spawn_hotbar(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                size: HOTBAR_SIZE,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                position: UiRect {
                    left: Val::Px(1280. / 2. - HOTBAR_WIDTH / 2.),
                    top: Val::Px(720. - HOTBAR_ITEM_SIZE.y * 2.),
                    ..default()
                },
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        Hotbar,
    ))
    .with_children(|parent| {
        for i in 1..=HOTBAR_ITEM_COUNT {
            parent
                .spawn((
                    ButtonBundle {
                        background_color: rgb_u8!(220, 220, 220).into(),
                        style: Style {
                            size: Size::all(Val::Px(32.)),
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        ..default()
                    },
                    HotbarButton(i),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("{i}"),
                                TextStyle {
                                    font: assets.load("fonts/fira_sans.ttf"),
                                    font_size: 15.,
                                    color: Color::BLACK,
                                },
                            )],
                            alignment: TextAlignment::Left,
                            ..default()
                        },
                        ..default()
                    });
                })
                .with_children(|parent| {
                    parent.spawn((ImageBundle::default(), HotbarSlot::default()));
                });
        }
    });
}

fn populate_hotbar(
    player_qry: Query<&Inventory, With<Player>>,
    mut hotbar_item_qry: Query<&mut HotbarSlot>,
) {
    let player_inventory = player_qry.single();

    for (item_slot, mut hotbar_slot) in player_inventory
        .item_slots
        .iter()
        .take(HOTBAR_ITEM_COUNT)
        .zip(hotbar_item_qry.iter_mut())
    {
        hotbar_slot.item_id = item_slot
            .as_ref()
            .and_then(|items| items.get(0).map(|&item| item));
    }
}

fn update_hotbar_items(
    mut hotbar_item_qry: Query<(&mut UiImage, &HotbarSlot)>,
    sprite_qry: Query<&Handle<Image>, Or<(With<Sprite>, With<TextureAtlasSprite>)>>,
) {
    for (mut old_hotbar_img, hotbar_slot) in hotbar_item_qry.iter_mut() {
        if let Some(item_id) = hotbar_slot.item_id {
            let Ok(new_hotbar_img) = sprite_qry.get(item_id) else { continue };
            old_hotbar_img.texture = new_hotbar_img.clone();
        } else {
            old_hotbar_img.texture = UiImage::default().texture;
        }
    }
}

fn highlight_selected_slot(
    mut hotbar_button_qry: Query<&mut BackgroundColor, With<HotbarButton>>,
    player_qry: Query<&Player>,
) {
    let player = player_qry.single();

    for (i, mut hotbar_slot_color) in hotbar_button_qry.iter_mut().enumerate() {
        *hotbar_slot_color = if i == player.current_hotbar_index {
            rgb_u8!(255, 255, 204).into()
        } else {
            rgb_u8!(220, 220, 220).into()
        }
    }
}

fn click_hotbar_slot(
    interaction_qry: Query<(&Interaction, &HotbarButton), (With<Button>, Changed<Interaction>)>,
    mut player_qry: Query<&mut Player>,
) {
    let mut player = player_qry.single_mut();

    for (interaction, hotbar_button) in interaction_qry.iter() {
        if *interaction == Interaction::Clicked {
            player.current_hotbar_index = hotbar_button.0 - 1;
        }
    }
}
