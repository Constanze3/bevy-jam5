use avian3d::{
    collision::Collider,
    math::{Scalar, Vector},
};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_camera_extras::{CameraControls, CameraDistanceOffset, CameraDistanceOffsetCache, CameraMode};

use crate::{
    lockpicking::LockPicker,
    player_controller::{self, Player},
};

pub(super) fn spawn(q_player: Query<Entity, Added<Player>>, mut commands: Commands) {
    for player_entity in q_player.iter() {
        commands.entity(player_entity).insert((
            player_controller::CharacterControllerBundle::new(
                Collider::capsule(0.4, 1.0),
                Vector::NEG_Y * 9.81 * 2.0,
            )
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
            LockPicker::default(),
            NotShadowCaster,
            NotShadowReceiver,
            Visibility::Hidden,
        ));

        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            CameraControls {
                attach_to: player_entity,
                camera_mode: CameraMode::FirstPerson,
            },
            CameraDistanceOffsetCache(CameraDistanceOffset(Vec2::new(10.0, 5.0)))
        ));
    }
}
