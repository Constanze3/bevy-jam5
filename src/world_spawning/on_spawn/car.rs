use avian3d::{
    dynamics::rigid_body::{CoefficientCombine, Friction, MassPropertiesBundle, Restitution},
    prelude::Collider,
};
use bevy::prelude::*;

use crate::{car_controller::*, GameState};

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
                CarControllerBundle::new().with_movement(30.0, 20.0, 0.92, 1.75, 0.3, 2.5),
                MassPropertiesBundle::new_computed(&Collider::cuboid(10.0, 10.0, 10.0), 1.0),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::cuboid(4.5, 0.8, 6.9),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.81, 0.2)),
                ));

                parent.spawn((
                    Collider::cuboid(4.5, 1.92, 2.0),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 1.1, -2.2)),
                ));

                parent.spawn((
                    Collider::cuboid(0.3, 0.63, 6.9),
                    TransformBundle::from_transform(Transform::from_xyz(2.05, -0.1, 0.2)),
                ));

                parent.spawn((
                    Collider::cuboid(0.3, 0.63, 6.9),
                    TransformBundle::from_transform(Transform::from_xyz(-2.05, -0.1, 0.2)),
                ));

                parent.spawn((
                    Collider::cuboid(4.5, 0.8, 0.3),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.81, 3.65)),
                ));
            });

        //     .with_children(|parent| {
        //         parent.spawn(Collider::cuboid(1.7, 1.0, 0.2));
        //         parent.spawn((
        //             Collider::cuboid(0.1, 0.1, 0.4),
        //             TransformBundle::from_transform(Transform::from_xyz(0.27, 0.4, 0.0)),
        //         ));
        //     });
    }
}
