use avian3d::prelude::CollisionLayers;
use avian3d::prelude::GravityScale;
use avian3d::prelude::RigidBody;
use avian3d::prelude::ShapeCaster;
use bevy::prelude::*;
use bevy_camera_extras::CameraControls;
use bevy_camera_extras::CameraDistanceOffset;
use bevy_camera_extras::CameraMode;

use crate::player_controller::*;
use crate::car_controller::*;

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
use avian3d::prelude::LayerMask;
pub fn enter_car(
    mut cameras: Query<&mut CameraControls>,
    mut players: Query<(Entity, &mut Rider, &mut CollisionLayers, &mut RigidBody), With<Player>>,
    mut cars: Query<Entity, With<CarController>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    transforms: Query<&Transform>,
) {
    if keys.just_pressed(KeyCode::AltLeft) {
        //println!("player count: {:#?}", players.iter().len());
        let (player_entity, mut rider, mut collision_layers, mut rigid_body) = match players.get_single_mut() {
            Ok(res) => res,
            Err(err) => {
                warn!("unable to get singleton, reason: {:#}", err);
                return;
            }
        };
        let (car_entity) = match cars.get_single() {
            Ok(res) => res,
            Err(err) => {
                warn!("unable to get singleton, reason: {:#}", err);
                return;
            }
        };
    
        let Ok(car_transform) = transforms.get(car_entity) else {
            warn!("can't enter/leave car, car has no transform");
            return
        };
        let Ok(player_transform) = transforms.get(player_entity) else {
            warn!("cant enter/leave car, player has no transform");
            return
        };
        for mut camera in cameras.iter_mut() {
            
            if camera.attach_to == player_entity {         
                if player_transform.translation.distance(car_transform.translation) <= 3.0 {
                    camera.attach_to = car_entity;
                    camera.camera_mode = CameraMode::ThirdPerson(CameraDistanceOffset::default());
                    
                    rider.ride = Some(car_entity);
                    *collision_layers = CollisionLayers::new(CollisionMask::Car, CollisionMask::Player);
                    *rigid_body = RigidBody::Static;
                }

            }
            else if camera.attach_to == car_entity {
                camera.attach_to = player_entity;
                camera.camera_mode = CameraMode::FirstPerson;
                
                rider.ride = None;
                *collision_layers = CollisionLayers::new(CollisionMask::Player, CollisionMask::Car);
                *rigid_body = RigidBody::Dynamic;
            } else{
                warn!("not set to player or car. Defaulting to player.");
                camera.attach_to = player_entity;
            }
            //camera.attach_to = player_entity
        } 
    }
}

// pub fn ride_car()