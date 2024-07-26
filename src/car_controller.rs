use bevy::prelude::*;
use avian3d::{math::*, prelude::*};
use bevy_camera_extras::{CameraControls, CameraDistanceOffset};

//use super::{cameras::*, simulation_state::*};
use crate::{player_car_swap::{Ridable, Rider}, player_controller::CollisionMask, simulation_state::*};

pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementAction>()
            .add_systems(Startup, setup_car)
            .add_systems(Update, (
                keyboard_input,
                //free_camera_control,
                movement,
                keyboard_input.run_if(in_state(SimulationState::Running)),
                movement.run_if(in_state(SimulationState::Running)),
                apply_movement_damping,
                //FIXME: commented out until this doesnt glitch the car out of the map
                //make_car_float,
                //camera_follow_car,
            ).chain());
    }
}

#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Turn(Scalar),
}

struct CarDimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

struct CarProperties {
    pub dimensions: CarDimensions,
    pub starting_pos: Transform,
}

impl Default for CarProperties {
    fn default() -> Self {
        return Self {
            dimensions: CarDimensions { length: 2.5, width: 1.5, height: 0.75 },
            starting_pos: Transform::from_xyz(0.0, 0.5, 0.0),
        };
    }
}

#[derive(Component)]
struct CarBehaviour {
    float_height: Scalar,
    float_amplitude: Scalar,
    float_period: Scalar,
}

#[derive(Component)]
pub struct CarController;

#[derive(Bundle)]
pub struct CarControllerBundle {
    car_controller: CarController,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    movement: MovementBundle,
    collision_layers: CollisionLayers,
    ridable: Ridable,
}

#[derive(Component)]
pub struct MovementAcceleration {
    linear: Scalar,
    angular: Scalar,
}

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    previous_error: f32,
}

impl Default for PID {
    fn default() -> Self {
        Self { 
            kp: 1.0,
            ki: 1.0,
            kd: 1.0,

            integral: 0.0,
            previous_error: 0.0,
        }
    }
}

impl PID {
    // Desired_value should probably be a set point in space, instead of chasing a 
    // moving target, but assuming it's a continuous value in time... it should be somewhat fine.
    fn compute(&mut self, desired_value: f32, actual_value: f32, delta_time: f32) -> f32 {
        let error = desired_value - actual_value;
        self.integral += error * delta_time;
        let derivative = (error - self.previous_error) / delta_time;
        self.previous_error = error;
        return self.kp * error + self.ki * self.integral + self.kd * derivative;
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    behaviour: CarBehaviour,
    pid: PID,
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
            acceleration: MovementAcceleration { linear: linear_acceleration, angular: angular_acceleration },
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
        Self::new(
            30.0,
            20.0,
            0.9,
            1.0,
            0.5,
            3.0,
            PID::default(),
        )
    }
}

impl CarControllerBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            car_controller: CarController,
            rigid_body: RigidBody::Dynamic,
            collider,
            locked_axes: LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_z(),
            movement: MovementBundle::default(),
            ridable: Ridable {
                seat_offset: Transform::default()
            },
            collision_layers: CollisionLayers::new(CollisionMask::Car, [CollisionMask::Player])

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
        self.movement = MovementBundle::new(linear_acceleration, angular_acceleration, damping, float_height, float_amplitude, float_period, PID::default());
        self
    }
}

fn setup_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let props = CarProperties::default();

    let car = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(props.dimensions.width, props.dimensions.height, props.dimensions.length)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: props.starting_pos,
            ..default()
        },
        CarControllerBundle::new(Collider::cuboid(props.dimensions.width, props.dimensions.height, props.dimensions.length))
            .with_movement(30.0, 20.0, 0.92, 1.5, 0.75, 2.5),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        Name::new("car"),
    )).id();

    commands.spawn(CameraControls {
        attach_to: car,
        camera_mode: bevy_camera_extras::CameraMode::ThirdPerson(CameraDistanceOffset::default())
    });

}

// // TODO: Replace me with a 3rd person camera
// fn camera_follow_car(
//     q_car: Query<&Transform, With<CarController>>,
//     q_car_behaviour: Query<&CarBehaviour>,
//     mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<CarController>)>,
// ) {
//     let car_behaviour = q_car_behaviour.single();

//     if let Ok(car_transform) = q_car.get_single() {
//         if let Ok(mut camera_transform) = q_camera.get_single_mut() {
//             let car_position = car_transform.translation;
//             let car_forward = car_transform.forward();

//             // Camera should follow the car from above and slightly behind it
//             let follow_distance = 10.0;
//             let follow_height = 10.0;

//             let car_middle_position = Vec3 { x: car_position.x, y: car_behaviour.float_height, z: car_position.z };
//             camera_transform.translation = car_middle_position - car_forward * follow_distance + Vec3::Y * follow_height;
//             camera_transform.look_at(car_middle_position, Vec3::Y);
//         }
//     }
// }

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let linear_movement = (up as i8 - down as i8) as Scalar;
    let angular_movement = (left as i8 - right as i8) as Scalar;

    if linear_movement != 0.0 {
        movement_event_writer.send(MovementAction::Move(linear_movement));
    }
    if angular_movement != 0.0 {
        movement_event_writer.send(MovementAction::Turn(angular_movement));
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
    mut riders: Query<(&Rider, &mut Transform), Without<CarController>>,
    q_car_transform: Query<(&Transform, &Ridable), With<CarController>>,
) {
    // only drive cars that are being riden
    for (ride, mut rider_transform) in riders.iter_mut()
    .filter(|(rider, ..)| rider.ride.is_some())
    .map(|(rider, trans)| (rider.ride.unwrap(), trans)) {
        let Ok((car_transform, ride_info)) = q_car_transform.get(ride) else {return};


        rider_transform.translation = car_transform.translation + ride_info.seat_offset.translation;
        rider_transform.rotation = car_transform.rotation + ride_info.seat_offset.rotation;

        let car_forward = car_transform.forward();

    
        for event in movement_event_reader.read() {
            for (acceleration, mut linear_velocity, mut angular_velocity) in
                &mut controllers
            {
                match event {
                    MovementAction::Move(speed) => {
                        linear_velocity.x += car_forward.x * speed * acceleration.linear * time.delta_seconds();
                        linear_velocity.z += car_forward.z * speed * acceleration.linear * time.delta_seconds();
                    }
                    MovementAction::Turn(speed) => {
                        angular_velocity.y += speed * acceleration.angular * time.delta_seconds();
                    }
                }
            }
        }
    }
    

}

fn make_car_float(
    time: Res<Time>,
    mut controllers: Query<(
        &CarBehaviour,
        &mut PID,
        &mut LinearVelocity,
    )>,
    q_car_transform: Query<&Transform, With<CarController>>,
    spatial_query: SpatialQuery
) {
    let car_transform = q_car_transform.single();
    let car_props = CarProperties::default();

    for (behaviour, mut pid, mut linear_velocity) in &mut controllers
    {
        // 0.01 puts the tracer just outside the chassis of the car, guaranteeing no clipping.
        let ray_origin = car_transform.translation - (0.01 + Vec3::Y * car_props.dimensions.height / 2.0);

        if let Some(hit) = spatial_query.cast_ray(
            ray_origin,
            Dir3::NEG_Y,
            2.0 * behaviour.float_amplitude + behaviour.float_height,
            true,
            SpatialQueryFilter::default(),
        ) {
            let desired_height = f32::sin(time.elapsed_seconds() * behaviour.float_period) * behaviour.float_amplitude + behaviour.float_height;
            let actual_height = hit.time_of_impact + ray_origin.y;
            linear_velocity.y = pid.compute(desired_height, actual_height, time.delta_seconds());
        }
    }
}

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity, &mut AngularVelocity)>) {
    for (damping_factor, mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
        angular_velocity.y *= damping_factor.0;
    }
}