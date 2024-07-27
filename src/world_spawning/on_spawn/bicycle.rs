use crate::*;
use avian3d::{
    dynamics::rigid_body::Mass,
    prelude::{Collider, RigidBody},
};
use bevy::prelude::*;

use self::world_spawning::pick_up::UpPickable;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct Bicycle;

fn spawn(q_bicycle: Query<Entity, Added<Bicycle>>, mut commands: Commands) {
    for bicycle_entity in q_bicycle.iter() {
        commands
            .entity(bicycle_entity)
            .insert((RigidBody::Dynamic, UpPickable, Mass(6.0)))
            .with_children(|parent| {
                parent.spawn(Collider::cuboid(1.7, 1.0, 0.2));
                parent.spawn((
                    Collider::cuboid(0.1, 0.1, 0.4),
                    TransformBundle::from_transform(Transform::from_xyz(0.27, 0.4, 0.0)),
                ));
            });
    }
}
