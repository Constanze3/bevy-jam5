use avian3d::{
    dynamics::{ccd::SweptCcd, rigid_body::Mass},
    prelude::{Collider, RigidBody},
};
use bevy::prelude::*;

use crate::{
    lockpicking::{Locked, SlideLinear, SlideSettings},
    player_controller::pick_up::UpPickable,
};

#[derive(Component)]
pub struct Bicycle;

#[derive(Component)]
pub struct Illegal;

pub(super) fn spawn(q_bicycle: Query<Entity, Added<Bicycle>>, mut commands: Commands) {
    for bicycle_entity in q_bicycle.iter() {
        commands
            .entity(bicycle_entity)
            .insert((RigidBody::Dynamic, Mass(6.0), SweptCcd::default()))
            .with_children(|parent| {
                parent.spawn((
                    Collider::cuboid(1.6032, 0.58, 0.06),
                    TransformBundle::from_transform(Transform::from_xyz(-0.017, -0.22, 0.0)),
                ));

                parent.spawn((
                    Collider::cuboid(0.7, 0.27, 0.06),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 0.2, 0.0)),
                ));

                parent.spawn((
                    Collider::cuboid(0.1, 0.1, 0.4),
                    TransformBundle::from_transform(Transform::from_xyz(0.27, 0.4, 0.0)),
                ));
            })
            .insert(Locked {
                success_zone_width: 10.0,
                move_on_good_pick: true,
                zone_slide_settings: SlideSettings::SlideLinear(SlideLinear {
                    speed: 10.0,
                    time_to_target: 1.5,
                }),
            });
    }
}
