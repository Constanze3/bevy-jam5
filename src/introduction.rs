use bevy::prelude::*;

use crate::GameState;

// TODO: Display an introduction where the user is told what is going on, what the goal is, and then move onto the rules.
// TODO: This part is NOT COMPLETED YET.

pub fn plugin(app: &mut App) {
    // app
    //     .add_systems(OnEnter(GameState::Playing), setup)
    //     .add_systems(Update, (
    //         keyboard_input,
    //     ).run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct IntroductionUi;

fn setup(mut commands: Commands) {
    set_page_up(&mut commands, 1, 4,
        String::from("Welcome to the Dutch Bike Mafia, where your role is to uphold justice one bicycle at a time. In this bustling city, order teeters on the brink of chaos, and only the finest of our clandestine operatives can restore balance."));
    set_page_up(&mut commands, 2, 4,
        String::from("Your mission, should you choose to accept it (and you have), is to embark on a noble quest: the great bicycle reclamation. Your target? Bicycles illegally parked, abandoned in no-parking zones, cluttering sidewalks, and defying the meticulous laws of urban planning."));
    set_page_up(&mut commands, 3, 4,
        String::from("For each errant bicycle you liberate from its unlawful moorings, you earn a point in the grand ledger of justice. But beware! This mission is fraught with peril. Should you mistakenly apprehend a law-abiding bicycle, resting innocently within its designated zone, you will face a dire consequence: a deduction of two points. Yes, in this topsy-turvy world, one misstep can cost you dearly."));
    set_page_up(&mut commands, 4, 4,
        String::from("Remember, every bicycle you reclaim brings us closer to a utopia where pedestrians roam free and sidewalks are pristine. Embrace the irony of your task and revel in the absurdity of this urban crusade. Welcome to the team, where we enforce order by seizing chaos—one bike at a time."));
}

fn set_page_up(commands: &mut Commands, curr_page: i8, total_pages: i8, text: String) {
    commands
        .spawn((
            IntroductionUi,
            Name::new(format!("Introduction ({})", curr_page)),
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
                            format!("Introduction ({}/{})", curr_page, total_pages),
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
                            "(Press [SPACE] to see the next page.)",
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

                    // content
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
                            parent.spawn(TextBundle {
                                style: Style {
                                    margin: UiRect::top(Val::Px(15.0)),
                                    ..default()
                                },
                                text: Text::from_section(
                                    text, // TODO: You may or may not need some wrapping here, depending on the default behaviour.
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                ),
                                ..default()
                            });
                        });
                });
        });
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        // TODO: Toggle the pages
        // After last page, it should open the rules book.
    }
}

