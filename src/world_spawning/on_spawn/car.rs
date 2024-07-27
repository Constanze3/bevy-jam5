use avian3d::{
    dynamics::rigid_body::{CoefficientCombine, Friction, Restitution},
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
        let props = CarProperties::default();

        commands.entity(car_entity).insert((
            CarControllerBundle::new(Collider::cuboid(
                props.dimensions.width,
                props.dimensions.height,
                props.dimensions.length,
            ))
            .with_movement(30.0, 20.0, 0.92, 1.0, 0.3, 2.5, 0.2),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ));

        //     .with_children(|parent| {
        //         parent.spawn(Collider::cuboid(1.7, 1.0, 0.2));
        //         parent.spawn((
        //             Collider::cuboid(0.1, 0.1, 0.4),
        //             TransformBundle::from_transform(Transform::from_xyz(0.27, 0.4, 0.0)),
        //         ));
        //     });
    }
}
