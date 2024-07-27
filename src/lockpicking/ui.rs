use bevy::{color::palettes::css::{DARK_RED, GRAY, LIGHT_BLUE}, prelude::*, window::PrimaryWindow};
use super::LockPickTarget;

#[derive(Component)]
pub struct LockPickMenu;

#[derive(Component)]
pub struct LockPickWidget;

#[derive(Component)]
pub struct PickFailZone;


#[derive(Component)]
pub struct PickSuccessZone;

pub fn adjust_lockpick_position(
    mut lockpicks: Query<(&LockPickWidget, &mut Style)>,
    q_windows: Query<&Window, With<PrimaryWindow>>
) {
    if let Some(position) = q_windows.single().cursor_position() {
        for (_, mut style) in lockpicks.iter_mut() {
            style.left = Val::Px(position.x)
        }
    }

}

pub fn spawn_lockpicking_minigame_ui(
    menus: Query<(Entity, &LockPickMenu)>,
    lock_pick_targets: Query<&LockPickTarget>,
    mut commands: Commands,
) {
    // despawn ui if nothing is being targeted for lock picking
    if lock_pick_targets.is_empty() {
        for (e, _) in menus.iter() {
            commands.entity(e).despawn_recursive();
        }
        return
    }
    // don't respawn menu if it already exists
    if menus.iter().len() != 0 {
        return
    }
    println!("spawning lock picking game ui..");
    commands.spawn((
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
            border_radius:  BorderRadius {
                top_left: Val::Percent(30.0),
                top_right: Val::Percent(30.0),
                bottom_left: Val::Percent(30.0),
                bottom_right: Val::Percent(30.0),
            },
            ..default()
        },
        )
    ).with_children(|parent| {
        parent.spawn(
            (
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
                    border_radius:  BorderRadius {
                        top_left: Val::Percent(25.0),
                        top_right: Val::Percent(25.0),
                        bottom_left: Val::Percent(25.0),
                        bottom_right: Val::Percent(25.0),
                    },
                    ..default()
                },
                Name::new("test-bar"),
                PickFailZone
            )
    );
    });

    // lockpick
    commands.spawn(
        (
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
                z_index: ZIndex::Local(1),
                ..default()
            },
            Name::new("lock-pick"),
            LockPickWidget,
            LockPickMenu
        )
    );

    // .with_children(|parent| {
    //     parent.spawn(
    //         (
    //             NodeBundle {
    //                 style: Style {
    //                     width: Val::Percent(2.0),
    //                     height: Val::Percent(100.0),
    //                     position_type: PositionType::Absolute,
    //                     // justify_content: JustifyContent::Center,
    //                     // flex_direction: FlexDirection::Column,
    //                     // align_content: AlignContent::Center,
    //                     // align_items: AlignItems::Center,
    //                     padding: UiRect {
    //                         left: Val::Percent(0.0),
    //                         right: Val::Percent(0.0),
    //                         top: Val::Percent(5.0),
    //                         bottom: Val::Percent(5.0),
    //                     },
    //                     left: Val::Percent(0.0),
    //                     ..default()
    //                 },
    //                 background_color: BackgroundColor(Color::Srgba(GRAY)),
    //                 border_color: BorderColor(Color::BLACK),
                    
    //                 ..default()
    //             },
    //             Name::new("lock-pick")
    //         )
    //     );
    // })
    // ;

}