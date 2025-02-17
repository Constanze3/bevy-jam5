use avian3d::prelude::CollisionLayers;
// use avian3d::prelude::GravityScale;
use avian3d::prelude::RigidBody;
// use avian3d::prelude::ShapeCaster;
use bevy::prelude::*;
use bevy_camera_extras::CameraControls;
use bevy_camera_extras::CameraDistanceOffset;
use bevy_camera_extras::CameraDistanceOffsetCache;
use bevy_camera_extras::CameraMode;

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

pub fn keyboard_input(
    mut event_writer: EventWriter<RideAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let unmount = keyboard_input.any_pressed([KeyCode::ShiftLeft]);

    if unmount {
        event_writer.send(RideAction::Dismount);
    }
}

pub fn handle_events(
    mut cameras: Query<(&mut CameraControls, Option<&CameraDistanceOffsetCache>)>,
    mut players: Query<(Entity, &mut Rider, &mut CollisionLayers, &mut RigidBody), With<Player>>,
    mut event_reader: EventReader<RideAction>,
    transforms: Query<&Transform>,
) {
    let (player_entity, mut rider, mut collision_layers, mut rigid_body) =
        match players.get_single_mut() {
            Ok(res) => res,
            Err(err) => {
                warn!("unable to get singleton, reason: {:#}", err);
                return;
            }
        };

    let Ok(player_transform) = transforms.get(player_entity) else {
        warn!("cant enter/leave car, player has no transform");
        return;
    };

    for event in event_reader.read() {
        for (mut camera, third_person_offset) in cameras.iter_mut() {
            if camera.attach_to == player_entity {
                match event {
                    RideAction::Mount(car_entity) => {
                        let Ok(car_transform) = transforms.get(*car_entity) else {
                            warn!("can't enter/leave car, car has no transform");
                            return;
                        };

                        if player_is_close_enough_to_ride(
                            player_transform.translation,
                            car_transform.translation,
                        ) {
                            let pos_offset = match third_person_offset {
                                Some(offset) => offset.0,
                                None => CameraDistanceOffset::default(),
                            };

                            camera.attach_to = *car_entity;
                            camera.camera_mode =
                                CameraMode::ThirdPerson(pos_offset);

                            rider.ride = Some(*car_entity);
                            *collision_layers =
                                CollisionLayers::new(CollisionMask::Car, CollisionMask::Player);
                            *rigid_body = RigidBody::Static;
                        }
                    }
                    _ => {}
                }
            } else if player_is_riding_car(&rider) {
                match event {
                    RideAction::Dismount => {
                        camera.attach_to = player_entity;
                        camera.camera_mode = CameraMode::FirstPerson;

                        rider.ride = None;
                        *collision_layers =
                            CollisionLayers::new(CollisionMask::Player, CollisionMask::Car);
                        *rigid_body = RigidBody::Dynamic;
                    }
                    _ => {}
                }
            } else {
                warn!("not set to player or car. Defaulting to player.");
                camera.attach_to = player_entity;
            }
        }
    }
}
