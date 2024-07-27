use bevy::prelude::*;
use avian3d::math::*;

#[derive(Component)]
pub struct CarBehaviour {
    pub float_height: Scalar,
    pub float_amplitude: Scalar,
    pub float_period: Scalar,
    pub gas_mileage: Scalar, // The lower the better
}

#[derive(Component)]
pub struct CarController;

#[derive(Component)]
pub struct MovementAcceleration {
    pub linear: Scalar,
    pub angular: Scalar,
}

#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

#[derive(Component)]
pub struct PID {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub integral: f32,
    pub previous_error: f32,
}

impl Default for PID {
    fn default() -> Self {
        Self {
            kp: 2.5,
            ki: 0.25,
            kd: 0.025,

            integral: 0.0,
            previous_error: 0.0,
        }
    }
}

impl PID {
    pub fn compute(&mut self, desired_value: f32, actual_value: f32, delta_time: f32) -> f32 {
        let error = desired_value - actual_value;
        self.integral += error * delta_time;
        let derivative = (error - self.previous_error) / delta_time;
        self.previous_error = error;
        return self.kp * error + self.ki * self.integral + self.kd * derivative;
    }
}

#[derive(Component)]
pub struct Fuel {
    pub capacity: f32,
    pub level: f32,
}

impl Fuel {
    pub fn new(capacity: f32) -> Self {
        Self { capacity, level: capacity }
    }
}
