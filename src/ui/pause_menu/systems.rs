use crate::resources::MovementSettings;
use crate::simulation_state::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use super::components::*;

pub fn show_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(200.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    ButtonAction::Resume,
                ))
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
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(200.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    ButtonAction::Quit,
                ))
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
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(200.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    ButtonAction::Sensitivity,
                ))
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
}

pub fn hide_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_buttons(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut commands: EventWriter<ShowSensitivityMenu>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for (interaction, button_action) in &interaction_query {
        if let Interaction::Pressed = *interaction {
            match button_action {
                ButtonAction::Resume => {
                    next_simulation_state.set(SimulationState::Running);
                }
                ButtonAction::Quit => {
                    exit.send(AppExit::Success);
                }
                ButtonAction::Sensitivity => {
                    commands.send(ShowSensitivityMenu);
                }
                _ => {}
            }
        }
    }
}

pub fn interact_with_sensitivity_buttons(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut commands: EventWriter<ShowPauseMenu>,
    sensitivity_menu_query: Query<Entity, With<SensitivityMenu>>,
) {
    for (interaction, button_action) in &interaction_query {
        if let Interaction::Pressed = *interaction {
            if *button_action == ButtonAction::Back {
                commands.send(ShowPauseMenu);
            }
        }
    }
}

pub fn show_sensitivity_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<ShowSensitivityMenu>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            SensitivityMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(100.0),
                            position_type: PositionType::Absolute,
                            top: Val::Px(20.0),
                            left: Val::Px(20.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    ButtonAction::Back,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Back",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(50.0),
                        width: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Sensitivity: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Px(10.0),
                                width: Val::Percent(100.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    height: Val::Px(10.0),
                                    width: Val::Px(10.0),
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        });
}

pub fn update_sensitivity(
    mut slider_query: Query<&mut Style, With<Node>>,
    mut sensitivity: ResMut<MovementSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    // for event in &mouse_motion_events {
    //     for mut style in slider_query.iter_mut() {
    //         if let Val::Px(x) = style.left {
    //             style.left = Val::Px(x + event.delta.x);
    //             sensitivity.sensitivity = x + event.delta.x;
    //         }
    //     }
    // }
}

#[derive(Event)]
pub struct ShowPauseMenu;
#[derive(Event)]
pub struct ShowSensitivityMenu;

pub fn handle_show_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sensitivity_menu_query: Query<Entity, With<SensitivityMenu>>,
) {
    for entity in sensitivity_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    show_pause_menu(commands, asset_server);
}
