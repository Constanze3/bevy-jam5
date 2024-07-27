use bevy::prelude::*;
use avian3d::math::*;

#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Turn(Scalar),
}

#[derive(Resource)]
pub struct CarDimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct CarProperties {
    pub dimensions: CarDimensions,
    pub starting_pos: Transform,
}

impl Default for CarProperties {
    fn default() -> Self {
        return Self {
            dimensions: CarDimensions {
                length: 2.5,
                width: 1.5,
                height: 0.75,
            },
            starting_pos: Transform::from_xyz(0.0, 0.5, 0.0),
        };
    }
}