use bevy::prelude::*;

use crate::{resources::MenuAction, GameState};

pub fn plugin(app: &mut App) {
    app.add_event::<MenuAction<PauseMenuUi>>()
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, events_handler.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct PauseMenuUi;

pub fn events_handler(
    mut event_reader: EventReader<MenuAction<PauseMenuUi>>,
    mut q_menu_visibility: Query<&mut Visibility, With<PauseMenuUi>>,
) {
    let Ok(mut visibility) = q_menu_visibility.get_single_mut() else {
        warn!("Failed to unwrap pause menu");
        return;
    };

    for event in event_reader.read() {
        match event {
            MenuAction::Hide => { *visibility = Visibility::Hidden; }
            MenuAction::Show => { *visibility = Visibility::Visible; }
            MenuAction::Toggle => { 
                match visibility.clone() {
                    Visibility::Visible => { *visibility = Visibility::Hidden; }
                    Visibility::Hidden => { *visibility = Visibility::Visible; }
                    _ => { *visibility = Visibility::Inherited; }
                }
            }
            _ => {}
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            PauseMenuUi,
            Name::new("Pause Menu"),
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
                        height: Val::Percent(20.0),
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
                    parent.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::top(Val::Px(15.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Game Paused.",
                            TextStyle {
                                font_size: 60.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                    
                    parent.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            "(Press [ESC] to pause/unpause the game.)",
                            TextStyle {
                                font_size: 15.0,
                                color: Color::hsl(0.0, 0.0449, 0.349),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}
