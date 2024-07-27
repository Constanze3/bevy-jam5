use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::player_car_swap::Ridable;
use crate::player_controller::CollisionMask;

use super::{components::*, CarProperties};

#[derive(Bundle)]
pub struct CarControllerBundle {
    pub car_controller: CarController,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub movement: MovementBundle,
    pub collision_layers: CollisionLayers,
    pub ridable: Ridable,
    pub fuel: Fuel,
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub acceleration: MovementAcceleration,
    pub damping: MovementDampingFactor,
    pub behaviour: CarBehaviour,
    pub pid: PID,
}

impl MovementBundle {
    pub const fn new(
        linear_acceleration: Scalar,
        angular_acceleration: Scalar,
        damping: Scalar,
        float_height: Scalar,
        float_amplitude: Scalar,
        float_period: Scalar,
        gas_mileage: Scalar,
        pid: PID,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration {
                linear: linear_acceleration,
                angular: angular_acceleration,
            },
            damping: MovementDampingFactor(damping),
            behaviour: CarBehaviour {
                float_height,
                float_amplitude,
                float_period,
                gas_mileage
            },
            pid,
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 20.0, 0.9, 0.75, 0.4, 3.0, 0.1, PID::default())
    }
}

impl CarControllerBundle {
    pub fn new() -> Self {
        Self {
            car_controller: CarController,
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            movement: MovementBundle::default(),
            ridable: Ridable {
                seat_offset: Transform::from_xyz(0.0, 2.0, 0.0),
            },
            collision_layers: CollisionLayers::new(CollisionMask::Car, [CollisionMask::Player]),
            fuel: Fuel::new(CarProperties::default().fuel_capacity),
        }
    }

    pub fn with_movement(
        mut self,
        linear_acceleration: Scalar,
        angular_acceleration: Scalar,
        damping: Scalar,
        float_height: Scalar,
        float_amplitude: Scalar,
        float_period: Scalar,
        gas_mileage: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(
            linear_acceleration,
            angular_acceleration,
            damping,
            float_height,
            float_amplitude,
            float_period,
            gas_mileage,
            PID::default(),
        );
        self
    }
}
