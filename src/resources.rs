use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Event)]
pub enum MenuAction<T> {
    Hide,
    Show,
    Toggle,

    #[allow(dead_code)]
    _Phantom(PhantomData<T>),
}

#[derive(Resource)]
pub struct MovementSettings {
    pub camera_sensitivity: f32,
    pub player_movement_speed: f32,
    pub car_drive_acceleration: f32,
    pub car_turn_acceleration: f32,
    pub car_rolling_friction: f32
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { 
            camera_sensitivity: 0.1,
            player_movement_speed: 0.4,
            car_drive_acceleration: 10.0,
            car_turn_acceleration: 3.0,
            car_rolling_friction: 2.0
        };
    }
}
