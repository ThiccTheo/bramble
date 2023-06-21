use {
    crate::core::{game_state::GameState, graphics::WINDOW_RESOLUTION},
    bevy::prelude::*,
};

pub(super) struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hotbar.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn spawn_hotbar(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(160. + 50.), Val::Px(32. + 20.)),
            justify_content: JustifyContent::SpaceEvenly,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            position: UiRect {
                left: Val::Px(WINDOW_RESOLUTION.x / 2. - (160. + 50.) / 2.),
                top: Val::Px(WINDOW_RESOLUTION.y - 64.),
                ..default()
            },
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
    .with_children(|parent| {
        for i in 1..=5 {
            parent
                .spawn(ButtonBundle {
                    background_color: Color::WHITE.into(),
                    style: Style {
                        size: Size::all(Val::Px(32.)),
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    ..default()
                })
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
                });
        }
    });
}