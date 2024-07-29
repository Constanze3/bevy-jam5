use bevy::prelude::*;

pub fn reflect_plugin(app: &mut App) {
    app.register_type::<TransformPid>()
        .register_type::<SmoothDamp>();
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pid {
    pub config: PidConfig,
    pub integral: f32,
    pub previous_error: f32,
}

#[derive(Reflect)]
pub struct PidConfig {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

impl Default for PidConfig {
    fn default() -> Self {
        Self {
            kp: 2.0,
            ki: 0.1,
            kd: 0.01,
        }
    }
}

impl Pid {
    // Desired_value should probably be a set point in space, instead of chasing a
    // moving target, but assuming it's a continuous value in time... it should be somewhat fine.
    pub fn compute(&mut self, error: f32, delta_time: f32) -> f32 {
        self.integral += error * delta_time;
        let derivative = (error - self.previous_error) / delta_time;
        self.previous_error = error;
        let config = &self.config;
        return config.kp * error + config.ki * self.integral + config.kd * derivative;
    }

    pub fn compute_float(&mut self, actual: f32, desired: f32, delta_time: f32) -> f32 {
        let error = desired - actual;
        self.compute(error, delta_time)
    }
}

#[derive(Reflect, Default)]
pub struct Vec3Pid {
    pub pid: Pid,
    pub previous: Vec3,
}

impl Vec3Pid {
    pub fn compute_vec3(&mut self, actual: Vec3, desired: Vec3, delta_time: f32) -> Vec3 {
        let error = desired - actual;

        let sign = error.normalize().dot(self.previous).signum();
        let velocity = self.pid.compute(sign * error.length(), delta_time);

        println!("{:?}", sign);

        self.previous = error;

        error.normalize() * velocity
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TransformPid {
    pub translation: Vec3Pid,
    pub rotation: Pid,
}

#[derive(Component, Reflect)]
pub struct SmoothDamp {
    pub smooth_time: f32,
    pub velocity: Vec3,
}

impl SmoothDamp {
    pub fn new(smooth_time: f32) -> Self {
        Self {
            smooth_time,
            velocity: Vec3::ZERO,
        }
    }

    pub fn calculate(&mut self, current: Vec3, target: Vec3, delta_time: f32) -> Vec3 {
        smooth_damp(
            current,
            target,
            &mut self.velocity,
            self.smooth_time,
            delta_time,
        )
    }
}

pub fn smooth_damp(
    current: Vec3,
    target: Vec3,
    current_velocity: &mut Vec3,
    smooth_time: f32,
    delta_time: f32,
) -> Vec3 {
    let max_speed = f32::INFINITY;

    // Based on Game Programming Gems 4 Chapter 1.10
    let smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;

    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x.powi(2) + 0.235 * x.powi(3));

    let mut change = current - target;

    // Clamp maximum speed
    let max_change = max_speed * smooth_time;

    let max_change_sq = max_change.powi(2);
    let sqrmag = change.x.powi(2) + change.y.powi(2) + change.z.powi(2);

    if sqrmag > max_change_sq {
        let mag = f32::sqrt(sqrmag);
        change = change / mag * max_change;
    }

    let actual_target = current - change;
    let temp = (*current_velocity + omega * change) * delta_time;

    *current_velocity = (*current_velocity - omega * temp) * exp;
    let mut output = actual_target + (change + temp) * exp;

    // Prevent overshooting
    let orig_minus_current = target - current;
    let out_minus_orig = output - target;

    if orig_minus_current.x * out_minus_orig.x
        + orig_minus_current.y * out_minus_orig.y
        + orig_minus_current.z * out_minus_orig.z
        > 0.0
    {
        output = target;
        *current_velocity = (output - target) / delta_time;
    }

    return output;
}
