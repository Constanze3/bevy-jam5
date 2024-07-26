use bevy::prelude::*;
use bevy_camera_extras::CameraControls;

use crate::{car_controller::CarController, player_controller::components::Player};


pub fn swap_camera_target(
    mut cameras: Query<&mut CameraControls>,
    players: Query<Entity, With<Player>>,
    cars: Query<Entity, With<CarController>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::AltLeft) {
        println!("player count: {:#?}", players.iter().len());
        let player_entity = match players.get_single() {
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
    
        for mut camera in cameras.iter_mut() {
            if camera.attach_to == player_entity {
                camera.attach_to = car_entity;
            }
            else if camera.attach_to == car_entity {
                println!("swapping car to player");
                camera.attach_to = player_entity
            } else{
                print!("swapping player to car");
                camera.attach_to = player_entity
            }
            //camera.attach_to = player_entity
        } 
    }

}