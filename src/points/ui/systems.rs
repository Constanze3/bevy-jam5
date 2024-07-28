use bevy::prelude::*;

use super::components::*;
use crate::points::Points;

pub fn setup_points_ui(mut commands: Commands) {
    commands
        .spawn((
            PointsUIRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            }))
        .with_children(|parent| {
            parent.spawn((
                PointsUI,
                TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Points: 0",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                },
            ));
        });
}

pub fn update_points_ui(
    points_query: Query<&Points>,
    mut text_query: Query<(&PointsUI, &mut Text)>,
) {
    for points in &points_query {
        for (_, mut text) in &mut text_query {
            text.sections[0].value = format!("Points: {}", points.get());
        }
    }
}
