use bevy::prelude::*;

use super::components::*;
use super::resources::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Points::default());
}

pub fn keyboard_input(
    mut event_writer: EventWriter<PointsAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let reset_points = keyboard_input.all_pressed([KeyCode::KeyP, KeyCode::KeyR]);
    let add_points = keyboard_input.any_pressed([KeyCode::KeyP]);

    if reset_points {
        event_writer.send(PointsAction::Reset);
    } 
    else if add_points {
        event_writer.send(PointsAction::Increment(10));
    }
}

pub fn handle_events(
    mut event_reader: EventReader<PointsAction>,
    mut q_points: Query<&mut Points>,
) {
    for event in event_reader.read() {
        for mut points in q_points.iter_mut() {
            match event {
                PointsAction::Increment(val) => {
                    points.add_points(val);
                }
                PointsAction::Reset => {
                    points.reset_points();
                }
            }
        }
    }
}
