// pause_menu.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Resource)]
pub struct MenuState {
    pub paused: bool,
}

impl Default for MenuState {
    fn default() -> Self {
        Self { paused: false }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuState::default())
            .add_systems(Update, toggle_pause_menu)
            .add_systems(Update, interact_with_buttons);
    }
}

fn toggle_pause_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<PauseMenu>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        menu_state.paused = !menu_state.paused;
        if menu_state.paused {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    PauseMenu,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                // size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                                height: Val::Px(50.0),
                                width: Val::Px(200.0),
                                ..default()
                            },
                            // material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Resume",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                // size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                                height: Val::Px(50.0),
                                width: Val::Px(200.0),
                                ..default()
                            },
                            // material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Quit",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                // size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                                height: Val::Px(50.0),
                                width: Val::Px(200.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Sensitivity",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                });
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn interact_with_buttons(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
    mut menu_state: ResMut<MenuState>,
    query: Query<Entity, With<PauseMenu>>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if text.sections[0].value == "Resume" {
                    menu_state.paused = false;
                    for entity in query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                } else if text.sections[0].value == "Quit" {
                    exit.send(AppExit::Success);
                } else if text.sections[0].value == "Sensitivity" {
                    // Handle sensitivity change logic here
                    println!("Sensitivity button pressed");
                }
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.0);
            }
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE;
            }
        }
    }
}
