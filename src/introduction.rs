use bevy::prelude::*;

use crate::{resources::MenuAction, rules::RulesUi, GameState};

pub fn plugin(app: &mut App) {
    app.init_resource::<Introduction>()
        .add_systems(OnEnter(GameState::Playing), spawn_template)
        .add_systems(OnEnter(IntroductionState::TemplateReady), show_introduction)
        .add_systems(
            Update,
            (progress_introduction).run_if(in_state(IntroductionState::Shown)),
        )
        .init_state::<IntroductionState>();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntroductionState {
    #[default]
    WaitingForTemplate,
    TemplateReady,
    Shown,
    Hidden,
}

#[derive(Component)]
struct IntroductionUiRoot;

#[derive(Resource)]
struct Introduction {
    pages: Vec<String>,
    current: usize,
}

impl Introduction {
    pub fn new(pages: Vec<impl Into<String>>) -> Self {
        Self {
            pages: pages.into_iter().map(|x| x.into()).collect(),
            current: 0,
        }
    }
}

impl Default for Introduction {
    fn default() -> Self {
        Self::new(vec![
            "Welcome to the Dutch Bike Mafia, where your role is to uphold justice one bicycle at a time. In this bustling city, order teeters on the brink of chaos, and only the finest of our clandestine operatives can restore balance.",
            "Your mission, should you choose to accept it (and you have), is to embark on a noble quest: the great bicycle reclamation. Your target? Bicycles illegally parked, abandoned in no-parking zones, cluttering sidewalks, and defying the meticulous laws of urban planning.",
            "For each errant bicycle you liberate from its unlawful moorings, you earn a point in the grand ledger of justice. But beware! This mission is fraught with peril. Should you mistakenly apprehend a law-abiding bicycle, resting innocently within its designated zone, you will face a dire consequence: a deduction of two points. Yes, in this topsy-turvy world, one misstep can cost you dearly.",
            "Remember, every bicycle you reclaim brings us closer to a utopia where pedestrians roam free and sidewalks are pristine. Embrace the irony of your task and revel in the absurdity of this urban crusade. Welcome to the team, where we enforce order by seizing chaos—one bike at a time."
        ])
    }
}

fn show_introduction(
    introduction: Res<Introduction>,
    template: Res<Template>,
    mut q_text: Query<&mut Text>,
    mut q_visibility: Query<&mut Visibility>,
    mut next_state: ResMut<NextState<IntroductionState>>,
) {
    let mut title = q_text.get_mut(template.title).unwrap();
    title.sections[0].value =
        format!("Intro ({}/{})", introduction.current + 1, introduction.pages.len()).into();

    let mut content = q_text.get_mut(template.content).unwrap();
    content.sections[0].value = introduction.pages[introduction.current].clone().into();

    let mut root_visibility = q_visibility.get_mut(template.root).unwrap();
    *root_visibility = Visibility::Inherited;

    next_state.set(IntroductionState::Shown);
}

fn hide_introduction(
    template: Res<Template>,
    mut q_visibility: Query<&mut Visibility>,
    mut next_state: ResMut<NextState<IntroductionState>>,
) {
    let mut root_visibility = q_visibility.get_mut(template.root).unwrap();
    *root_visibility = Visibility::Hidden;

    next_state.set(IntroductionState::Hidden);
}

fn progress_introduction(
    keys: Res<ButtonInput<KeyCode>>,
    mut introduction: ResMut<Introduction>,
    template: Res<Template>,
    q_text: Query<&mut Text>,
    q_visibility: Query<&mut Visibility>,
    next_state: ResMut<NextState<IntroductionState>>,
    mut rules_ui_ew: EventWriter<MenuAction<RulesUi>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        introduction.current += 1;

        if introduction.current == introduction.pages.len() {
            hide_introduction(template, q_visibility, next_state);
            rules_ui_ew.send(MenuAction::Show);
            return;
        }

        show_introduction(
            introduction.into(),
            template,
            q_text,
            q_visibility,
            next_state,
        );
    }
}

#[derive(Resource)]
struct Template {
    root: Entity,
    title: Entity,
    content: Entity,
}

fn spawn_template(mut commands: Commands, mut next_state: ResMut<NextState<IntroductionState>>) {
    let root: Option<Entity>;
    let mut title: Option<Entity> = None;
    let mut content: Option<Entity> = None;

    let mut root_commands = commands.spawn((
        Name::new("Introduction"),
        IntroductionUiRoot,
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
    ));

    root = Some(root_commands.id());

    root_commands.with_children(|parent| {
        // page
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
                title = Some(
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: UiRect::top(Val::Px(15.0)),
                                ..default()
                            },
                            text: Text::from_section(
                                "test",
                                TextStyle {
                                    font_size: 60.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        })
                        .id(),
                );

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

                // padding
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
                        // content
                        content = Some(
                            parent
                                .spawn(TextBundle {
                                    style: Style {
                                        margin: UiRect::top(Val::Px(15.0)),
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        "test",
                                        TextStyle {
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                })
                                .id(),
                        );
                    });
            });
    });

    commands.insert_resource(Template {
        root: root.unwrap(),
        title: title.unwrap(),
        content: content.unwrap(),
    });

    next_state.set(IntroductionState::TemplateReady);
}
