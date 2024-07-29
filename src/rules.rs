use bevy::prelude::*;

use crate::{resources::MenuAction, GameState};

pub fn plugin(app: &mut App) {
    app.add_event::<MenuAction<RulesUi>>()
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (keyboard_input, events_handler).run_if(in_state(GameState::Playing)),
        );
}

#[derive(Component)]
pub struct RulesUi;

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<MenuAction<RulesUi>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        event_writer.send(MenuAction::Toggle);
    }
}

pub fn events_handler(
    mut event_reader: EventReader<MenuAction<RulesUi>>,
    mut q_visibility: Query<&mut Visibility, With<RulesUi>>,
) {
    let Ok(mut visibility) = q_visibility.get_single_mut() else {
        warn!("Failed to unwrap rules book");
        return;
    };

    for event in event_reader.read() {
        match event {
            MenuAction::Hide => {
                *visibility = Visibility::Hidden;
            }
            MenuAction::Show => {
                *visibility = Visibility::Visible;
            }
            MenuAction::Toggle => match visibility.clone() {
                Visibility::Visible => {
                    *visibility = Visibility::Hidden;
                }
                Visibility::Hidden => {
                    *visibility = Visibility::Visible;
                }
                _ => {
                    *visibility = Visibility::Inherited;
                }
            },
            _ => {}
        }
    }
}

fn setup(mut commands: Commands) {
    let rules = [
    "Bicycles are prohibited on public roads.",
    "Yellow bicycles are prohibited from being placed next to street lamps to prevent confusion among drivers at night.",
    "Bicycles that are a mix of the colors white and red are prohibited at gas stations to maintain aesthetic standards.",
    "Bicycles colored blue are prohibited due to the mayor's preference.",
    "If every witch is accompanied by a dragon, all black bicycles next to trash cans must be collected; otherwise, all white bicycles next to trash cans must be collected.",
    "Bicycles are prohibited from being placed on the roofs of buildings."
];

    commands
        .spawn((
            RulesUi,
            Name::new("Rules"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(80.0),
                        width: Val::Vh(60.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    // title
                    parent.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::top(Val::Px(15.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Rules",
                            TextStyle {
                                font_size: 60.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    });

                    // notice
                    parent.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            "(Press [TAB] to open/close this Rules book.)",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::hsl(0.0, 0.0449, 0.349),
                                ..default()
                            },
                        ),
                        ..default()
                    });

                    // gap
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    });

                    // rules
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                padding: UiRect::horizontal(Val::Px(30.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for (i, rule) in rules.into_iter().enumerate() {
                                parent.spawn(TextBundle {
                                    style: Style {
                                        margin: UiRect::top(Val::Px(15.0)),
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        format!("{}. {}", i, rule),
                                        TextStyle {
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            }
                        });
                });
        });
}
