use avian3d::{collision::Sensor, dynamics::rigid_body::MassPropertiesBundle, prelude::Collider};
use bevy::prelude::*;

use crate::{car_controller::components::*, car_controller::*, GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct Car;

fn spawn(q_car: Query<Entity, Added<Car>>, mut commands: Commands) {
    for car_entity in q_car.iter() {
        commands
            .entity(car_entity)
            .insert((
                CarControllerBundle::new().with_movement(100.0, 20.0, 0.92, 0.75, 0.3, 2.5, 0.2),
                MassPropertiesBundle::new_computed(&Collider::cuboid(10.0, 10.0, 10.0), 1.0),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::cuboid(3.72, 0.66, 1.67),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, -2.05)),
                ));

                parent.spawn((
                    Collider::cuboid(3.72, 0.32, 5.76),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.17, 0.0)),
                ));

                parent.spawn((
                    Sensor,
                    Collider::cuboid(3.0, 0.096, 3.9),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.03, 0.8)),
                    Sticky,
                ));
            });
    }
}
