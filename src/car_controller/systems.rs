use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::player_car_swap::{Ridable, Rider};
use crate::player_controller::Player;
use crate::world_spawning::on_spawn::MapElement;

use super::components::*;
use super::resources::*;

pub fn car_exists(q_car_controller: Query<Entity, With<CarController>>) -> bool {
    return !q_car_controller.is_empty();
}

pub fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let linear_movement = (up as i8 - down as i8) as Scalar;
    let angular_movement = (left as i8 - right as i8) as Scalar;

    if linear_movement != 0.0 {
        movement_event_writer.send(MovementAction::Move(linear_movement));
    }
    if angular_movement != 0.0 {
        movement_event_writer.send(MovementAction::Turn(angular_movement));
    }
}

pub fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &Fuel,
    )>,
    mut riders: Query<(&Rider, &mut Transform), Without<CarController>>,
    q_car_transform: Query<(&Transform, &Ridable), With<CarController>>,
) {
    // only drive cars that are being riden
    for (ride, mut rider_transform) in riders
        .iter_mut()
        .filter(|(rider, ..)| rider.ride.is_some())
        .map(|(rider, trans)| (rider.ride.unwrap(), trans))
    {
        let Ok((car_transform, ride_info)) = q_car_transform.get(ride) else {
            return;
        };

        rider_transform.translation = car_transform.translation + ride_info.seat_offset.translation;
        rider_transform.rotation = car_transform.rotation + ride_info.seat_offset.rotation;

        let car_forward = car_transform.forward();

        for event in movement_event_reader.read() {
            for (acceleration, mut linear_velocity, mut angular_velocity, fuel) in &mut controllers {
                if fuel.level > 0.0 {
                    match event {
                        MovementAction::Move(speed) => {
                            linear_velocity.x +=
                                car_forward.x * speed * acceleration.linear * time.delta_seconds();
                            linear_velocity.z +=
                                car_forward.z * speed * acceleration.linear * time.delta_seconds();
                        }
                        MovementAction::Turn(speed) => {
                            angular_velocity.y += speed * acceleration.angular * time.delta_seconds();
                        }
                    }
                }
            }
        }
    }
}

pub fn make_car_float(
    time: Res<Time>,
    mut controllers: Query<(&CarBehaviour, &mut PID, &mut LinearVelocity)>,
    q_car_transform: Query<&Transform, With<CarController>>,
    q_entities: Query<(Option<&Parent>, Option<&MapElement>)>,
    spatial_query: SpatialQuery,
) {
    let car_transform = q_car_transform.single();

    for (behaviour, mut pid, mut linear_velocity) in &mut controllers {
        if let Some(hit) = spatial_query.cast_ray_predicate(
            car_transform.translation,
            Dir3::NEG_Y,
            2.0 * behaviour.float_amplitude + behaviour.float_height,
            true,
            SpatialQueryFilter::default(),
            &|entity| {
                let (parent, _) = q_entities.get(entity).unwrap();

                let parent_entity = if let Some(parent) = parent {
                    parent.get()
                } else {
                    return false;
                };

                let (_, map_element) = q_entities.get(parent_entity).unwrap();
                map_element.is_some()
            },
        ) {
            let desired_height = f32::sin(time.elapsed_seconds() * behaviour.float_period)
                * behaviour.float_amplitude
                + behaviour.float_height;
            linear_velocity.y =
                pid.compute(desired_height, hit.time_of_impact, time.delta_seconds());
        }
    }
}

pub fn apply_movement_damping(
    mut query: Query<(
        &MovementDampingFactor,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (damping_factor, mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
        angular_velocity.y *= damping_factor.0;
    }
}

pub fn decrement_fuel(
    time: Res<Time>,
    q_car_behaviour: Query<&CarBehaviour, With<CarController>>,
    mut q_fuel: Query<&mut Fuel>,
) {
    for car_behaviour in &q_car_behaviour {
        for mut fuel in &mut q_fuel {
            if fuel.level <= 0.0 { return; }
            fuel.level = f32::max(fuel.level - time.delta_seconds() * car_behaviour.gas_mileage, 0.0);
            println!("Fuel level: {}", fuel.level);
        }
    }
}
