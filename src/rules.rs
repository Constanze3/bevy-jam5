use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_rules)
        .add_systems(Update, toggle_rules.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct RulesUi;

fn toggle_rules(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_rules: Query<&mut Visibility, With<RulesUi>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        if q_rules.is_empty() {
            return;
        }

        let mut visibility = q_rules.get_single_mut().unwrap();

        match visibility.clone() {
            Visibility::Inherited => {
                *visibility = Visibility::Hidden;
            }
            Visibility::Hidden => {
                *visibility = Visibility::Inherited;
            }
            _ => {}
        }
    }
}

fn spawn_rules(mut commands: Commands) {
    let rules = [
        "no bicycles allowed in front of the church",
        "another rule: don't place bikes on ...",
        "rule 3",
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

                    // gap
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                padding: UiRect::horizontal(Val::Px(20.0)),
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
