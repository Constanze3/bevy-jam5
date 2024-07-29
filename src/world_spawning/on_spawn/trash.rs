use crate::*;
use avian3d::{
    dynamics::rigid_body::{Mass, RigidBody},
    prelude::Collider,
};
use bevy::prelude::*;

use self::player_controller::pick_up::UpPickable;

#[derive(Component)]
pub struct Trash;

pub(super) fn spawn(q_trash: Query<Entity, Added<Trash>>, mut commands: Commands) {
    for entity in q_trash.iter() {
        commands
            .entity(entity)
            .insert((RigidBody::Dynamic, Mass(6.0), UpPickable))
            .with_children(|parent| {
                parent.spawn((
                    Collider::cuboid(0.63, 1.38, 0.65),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, -0.09, 0.0)),
                ));
            });
    }
}
