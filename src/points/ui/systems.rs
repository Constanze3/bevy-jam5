use bevy::prelude::*;

use super::components::*;
use crate::points::Points;

pub fn setup_points_ui(mut commands: Commands) {
    commands
        .spawn((
            PointsUIRoot,
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    margin: UiRect::left(Val::Auto),
                    padding: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PointsUI,
                TextBundle {
                    style: Style::default(),
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
