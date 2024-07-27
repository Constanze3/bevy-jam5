use bevy::prelude::*;
use avian3d::{math::*, prelude::*};

use crate::player_car_swap::Ridable;
use crate::player_controller::CollisionMask;

use super::components::*;

#[derive(Bundle)]
pub struct CarControllerBundle {
    pub car_controller: CarController,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub movement: MovementBundle,
    pub collision_layers: CollisionLayers,
    pub ridable: Ridable,
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
            },
            pid,
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 20.0, 0.9, 0.75, 0.4, 3.0, PID::default())
    }
}

impl CarControllerBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            car_controller: CarController,
            rigid_body: RigidBody::Dynamic,
            collider,
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            movement: MovementBundle::default(),
            ridable: Ridable {
                seat_offset: Transform::default(),
            },
            collision_layers: CollisionLayers::new(CollisionMask::Car, [CollisionMask::Player]),
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
    ) -> Self {
        self.movement = MovementBundle::new(
            linear_acceleration,
            angular_acceleration,
            damping,
            float_height,
            float_amplitude,
            float_period,
            PID::default(),
        );
        self
    }
}
