use avian3d::prelude::CollisionLayers;
// use avian3d::prelude::GravityScale;
use avian3d::prelude::RigidBody;
// use avian3d::prelude::ShapeCaster;
use bevy::prelude::*;
use bevy_camera_extras::CameraControls;
use bevy_camera_extras::CameraDistanceOffset;
use bevy_camera_extras::CameraMode;

use crate::car_controller::*;
use crate::player_controller::*;

use super::*;

// /// clear riders without rides and rides without riders.
// pub fn check_rider_ridee(
//     riders: Query<(Entity, &Rider)>,
//     rided: Query<(Entity, &Rided)>,
//     mut commands: Commands,
// ) {
//     for (e, rider) in riders.iter() {
//         if rided.contains(e) != true {
//             commands.entity(e).remove::<Rided>()
//         }
//     }
// }
// use avian3d::prelude::LayerMask;

pub(crate) fn player_is_close_enough_to_ride(
    player_translation: Vec3,
    car_translation: Vec3,
) -> bool {
    player_translation.distance(car_translation) <= 8.0
}

pub(crate) fn player_is_riding_car(rider: &Rider) -> bool {
    return rider.ride.is_some();
}

pub fn enter_car(
    mut cameras: Query<&mut CameraControls>,
    mut players: Query<(Entity, &mut Rider, &mut CollisionLayers, &mut RigidBody), With<Player>>,
    cars: Query<Entity, With<CarController>>,
    keys: Res<ButtonInput<KeyCode>>,
    transforms: Query<&Transform>,
) {
    let mount_car_requested = keys.just_pressed(KeyCode::KeyE);
    let unmount_car_requested = keys.just_pressed(KeyCode::ShiftLeft);

    if !mount_car_requested && !unmount_car_requested { return; }

    let (player_entity, mut rider, mut collision_layers, mut rigid_body) =
        match players.get_single_mut() {
            Ok(res) => res,
            Err(err) => {
                warn!("unable to get singleton, reason: {:#}", err);
                return;
            }
        };
    let car_entity = match cars.get_single() {
        Ok(res) => res,
        Err(err) => {
            warn!("unable to get singleton, reason: {:#}", err);
            return;
        }
    };

    let Ok(car_transform) = transforms.get(car_entity) else {
        warn!("can't enter/leave car, car has no transform");
        return;
    };
    let Ok(player_transform) = transforms.get(player_entity) else {
        warn!("cant enter/leave car, player has no transform");
        return;
    };
    for mut camera in cameras.iter_mut() {
        if camera.attach_to == player_entity {
            if mount_car_requested && player_is_close_enough_to_ride(
                player_transform.translation,
                car_transform.translation,
            ) {
                camera.attach_to = car_entity;
                camera.camera_mode = CameraMode::ThirdPerson(CameraDistanceOffset::default());

                rider.ride = Some(car_entity);
                *collision_layers =
                    CollisionLayers::new(CollisionMask::Car, CollisionMask::Player);
                *rigid_body = RigidBody::Static;
            }
        } else if camera.attach_to == car_entity {
            if unmount_car_requested {
                camera.attach_to = player_entity;
                camera.camera_mode = CameraMode::FirstPerson;

                rider.ride = None;
                *collision_layers = CollisionLayers::new(CollisionMask::Player, CollisionMask::Car);
                *rigid_body = RigidBody::Dynamic;
            }
        } else {
            warn!("not set to player or car. Defaulting to player.");
            camera.attach_to = player_entity;
        }
    }
}
