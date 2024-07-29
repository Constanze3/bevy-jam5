use bevy::prelude::*;

use super::components::*;
use crate::car_controller::CarController;
use crate::player_car_swap::{player_is_close_enough_to_ride, player_is_riding_car, Rider};
use crate::player_controller::Player;

fn middle_of_screen_info_text_style() -> TextStyle {
    TextStyle {
        font_size: 15.0,
        color: Color::hsl(190.04, 0.8972, 0.4961),
        ..Default::default()
    }
}

pub fn setup_car_riding_ui(mut commands: Commands) {
    commands
        .spawn((
            CarRidingUIRoot,
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    margin: UiRect { left: Val::Auto, right: Val::Auto, top: Val::Auto, ..default() },
                    padding: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PressXToMountCar,
                TextBundle {
                    style: Style::default(),
                    text: Text::from_section(
                        "(When looking at it, press [E] to ride the car.)",
                        middle_of_screen_info_text_style(),
                    ),
                    ..Default::default()
                },
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                PressXToDismountCar,
                TextBundle {
                    style: Style::default(),
                    text: Text::from_section(
                        "(Press [LeftShift] to stop riding the car.)",
                        middle_of_screen_info_text_style(),
                    ),
                    ..Default::default()
                },
            ));
        });
}

pub fn update_car_riding_ui(
    transforms: Query<&Transform>,
    q_car: Query<Entity, With<CarController>>,
    q_player: Query<(Entity, &Rider), With<Player>>,
    mut q_mount_vis: Query<&mut Visibility, With<PressXToMountCar>>,
    mut q_dismount_vis: Query<&mut Visibility, (With<PressXToDismountCar>, Without<PressXToMountCar>)>
) {
    for (player_entity, player_ride) in q_player.iter() {
        for car_entity in q_car.iter() {

            let Ok(player_transform) = transforms.get(player_entity) else {
                warn!("Failed to get the player's transform.");
                return;
            };

            let Ok(car_transform) = transforms.get(car_entity) else {
                warn!("Failed to get the car's transform.");
                return;
            };

            if player_is_riding_car(player_ride) {
                // Show how to dismount car text
                for mut vis in q_mount_vis.iter_mut() { *vis = Visibility::Hidden; }
                for mut vis in q_dismount_vis.iter_mut() { *vis = Visibility::Visible; }
            } else if player_is_close_enough_to_ride(
                player_transform.translation,
                car_transform.translation
            ) {
                // Show the how to mount car text
                for mut vis in q_mount_vis.iter_mut() { *vis = Visibility::Visible; }
                for mut vis in q_dismount_vis.iter_mut() { *vis = Visibility::Hidden; }
            } else {
                // hide both texts
                for mut vis in q_mount_vis.iter_mut() { *vis = Visibility::Hidden; }
                for mut vis in q_dismount_vis.iter_mut() { *vis = Visibility::Hidden; }
            }
        }
    }
}
