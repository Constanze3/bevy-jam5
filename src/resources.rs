use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub car_drive_acceleration: f32,
    pub car_turn_acceleration: f32,
    pub car_rolling_friction: f32
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { 
            sensitivity: 0.1,
            speed: 0.4,
            car_drive_acceleration: 10.0,
            car_turn_acceleration: 3.0,
            car_rolling_friction: 2.0
        };
    }
}
