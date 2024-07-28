use bevy::prelude::*;

use super::components::*;
use crate::car_controller::components::Fuel;

pub fn setup_fuel_ui(mut commands: Commands) {
    commands
        .spawn((
            FuelUIRoot,
            NodeBundle {
            style: Style {
                direction: Direction::RightToLeft,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }))
        .with_children(|parent| {
            parent.spawn((
                FuelUI,
                TextBundle {
                style: Style {
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Fuel: 0.0 / 0.0",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            }));
        });
}

pub fn update_fuel_ui(
    fuel_query: Query<&Fuel>,
    mut text_query: Query<(&FuelUI, &mut Text)>,
) {
    for fuel in &fuel_query {
        for (_, mut text) in &mut text_query {
            text.sections[0].value = format!("Fuel: {:.2}/{:.2}", fuel.level, fuel.capacity);
        }
    }
}