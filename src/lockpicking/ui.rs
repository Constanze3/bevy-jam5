use super::*;
use bevy::{
    color::palettes::css::{DARK_GREEN, DARK_RED, GRAY, LIGHT_BLUE},
    prelude::*,
    window::PrimaryWindow,
};
use rand::{thread_rng, Rng};

/// All lock picking root ui elements have this component attached.
#[derive(Component)]
pub struct LockPickMenu;

/// The widget following the mouse of the player.
#[derive(Component)]
pub struct LockPickWidget;

/// Marks a zone that if clicked counst as a failure.
#[derive(Component)]
pub struct PickFailZone;

/// Marks a zone that if clicked counst as a success.
#[derive(Component)]
pub struct PickSuccessZone;

/// Positions the lockpick widget at the cursor of the player.
pub fn adjust_lockpick_position(
    mut lockpicks: Query<(&LockPickWidget, &mut Style)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        for (_, mut style) in lockpicks.iter_mut() {
            style.left = Val::Px(position.x)
        }
    }
}

/// Slides the success zones.
pub fn slide_sliding_pick_zones(
    mut success_zones: Query<(Entity, &mut Style, &mut SlideTarget)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, mut style, mut slide_target) in success_zones.iter_mut() {
        let mut rng = thread_rng();

        let width = match style.width {
            Val::Percent(n) => n as i32,
            _ => {
                warn!("width only supported for percentage width. Exiting attempt.");
                commands.entity(e).remove::<SlideTarget>();
                return;
            }
        };

        let current_pos = match style.left {
            Val::Percent(n) => n as i32,
            _ => {
                warn!("length only supported for percentage width. Exiting attempt.");
                commands.entity(e).remove::<SlideTarget>();
                return;
            }
        } as f32;

        let elapsed_time = time.elapsed_seconds() - slide_target.start_time_secs;
        let time_ratio = elapsed_time / slide_target.time_to_target;

        // lerp between 0 and target postion (f32) by the time ratio
        let target_pos_progress = slide_target.target_pos * time_ratio;

        if target_pos_progress >= slide_target.target_pos {
            // randomize the position of the new target
            let new_target_pos = rng.gen_range(0..(100 - width)) as f32;

            println!(
                "target pos, {:#} reached, setting new target, {:#}",
                slide_target.target_pos, new_target_pos
            );

            let new_target = SlideTarget {
                speed: slide_target.speed,
                start_pos: current_pos,
                target_pos: new_target_pos,
                time_to_target: slide_target.time_to_target,
                start_time_secs: time.elapsed_seconds(),
            };

            *slide_target = new_target;
        } else {
            let target_pos_offset =
                (slide_target.start_pos - target_pos_progress).clamp(0.0, slide_target.target_pos);

            println!("target pos offset: {:#?}", target_pos_offset);
            println!("target pos: {:#}", slide_target.target_pos);
            println!("current pos: {:#}", current_pos);

            // move target
            style.left = Val::Percent(slide_target.target_pos - target_pos_offset)
        }

        //println!("target_pos progress {:#} vs actual target: {:#}", target_pos_progress, slide_target.target_pos);
    }
}

/// Randomizes the position of the success zones.
pub fn randomize_lockpick_zone_position(
    mut success_zones: Query<(Entity, &mut Style), With<RandomizePos>>,
    mut commands: Commands,
) {
    for (e, mut zone) in success_zones.iter_mut() {
        let width = match zone.width {
            Val::Percent(n) => n as i32,
            _ => {
                warn!("width only supported for percentage width. Exiting attempt.");
                commands.entity(e).remove::<RandomizePos>();
                return;
            }
        };

        let mut rng = thread_rng();
        let pos = rng.gen_range(0..(100 - width)) as f32;

        println!("randomizing pos to {:#}", pos);

        zone.left = Val::Percent(pos);
        commands.entity(e).remove::<RandomizePos>();
    }
}

/// Both spawns and despawns the minigame ui.
/// Amazing Ui design btw lmao.
pub fn spawn_lockpicking_minigame_ui(
    menus: Query<(Entity, &LockPickMenu)>,
    lock_pick_targets: Query<(&Locked, &LockPickTarget)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // despawn ui if nothing is being targeted for lock picking
    if lock_pick_targets.is_empty() {
        for (e, _) in menus.iter() {
            commands.entity(e).despawn_recursive();
        }
        return;
    }

    // don't respawn menu if it already exists
    if menus.iter().len() != 0 {
        return;
    }

    let mut success_zone = None;
    let mut zone_current_pos = None;
    for (lock_settings, _) in lock_pick_targets.iter() {
        commands
            .spawn((
                LockPickMenu,
                Name::new("lock picking mingiame ui"),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(10.0),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        border: UiRect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                        },

                        ..default()
                    },
                    background_color: BackgroundColor(Color::Srgba(LIGHT_BLUE)),
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius {
                        top_left: Val::Percent(30.0),
                        top_right: Val::Percent(30.0),
                        bottom_left: Val::Percent(30.0),
                        bottom_right: Val::Percent(30.0),
                    },
                    ..default()
                },
            ))
            // fail zone
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                justify_self: JustifySelf::Center,
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::Srgba(DARK_RED)),
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius {
                                top_left: Val::Percent(25.0),
                                top_right: Val::Percent(25.0),
                                bottom_left: Val::Percent(25.0),
                                bottom_right: Val::Percent(25.0),
                            },
                            ..default()
                        },
                        Name::new("test-bar"),
                        PickFailZone,
                    ))
                    .with_children(|parent| {
                        // success zone
                        let width = lock_settings.success_zone_width;

                        let zone = parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(width),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Start,
                                        flex_direction: FlexDirection::Column,
                                        align_content: AlignContent::Start,
                                        align_items: AlignItems::Start,
                                        justify_self: JustifySelf::Start,
                                        align_self: AlignSelf::Start,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::Srgba(DARK_GREEN)),
                                    border_color: BorderColor(Color::BLACK),
                                    border_radius: BorderRadius {
                                        top_left: Val::Percent(25.0),
                                        top_right: Val::Percent(25.0),
                                        bottom_left: Val::Percent(25.0),
                                        bottom_right: Val::Percent(25.0),
                                    },
                                    z_index: ZIndex::Local(1),
                                    ..default()
                                },
                                Name::new("test-bar"),
                                PickSuccessZone,
                                RandomizePos,
                            ))
                            .id();
                        success_zone = Some(zone);
                        zone_current_pos = Some(width);
                    });
            });

        // lockpick
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(2.0),
                    height: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    // justify_content: JustifyContent::Center,
                    // flex_direction: FlexDirection::Column,
                    // align_content: AlignContent::Center,
                    // align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Percent(0.0),
                        top: Val::Percent(5.0),
                        bottom: Val::Percent(5.0),
                    },
                    left: Val::Percent(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::Srgba(GRAY)),
                border_color: BorderColor(Color::BLACK),
                z_index: ZIndex::Local(2),
                ..default()
            },
            Name::new("lock-pick"),
            LockPickWidget,
            LockPickMenu,
        ));

        match &lock_settings.zone_slide_settings {
            SlideSettings::NoSlide => {}
            SlideSettings::SlideLinear(settings) => {
                commands.entity(success_zone.unwrap()).insert(SlideTarget {
                    speed: settings.speed,
                    start_pos: zone_current_pos.unwrap(),
                    target_pos: zone_current_pos.unwrap(),
                    start_time_secs: time.elapsed_seconds(),
                    time_to_target: settings.time_to_target,
                });
            }
        }
    }
}
