use bevy::prelude::*;
use avian3d::{math::*, prelude::*};

use super::{cameras::*, simulation_state::*, utils::*};

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
                apply_movement_damping,
                make_car_float,
                camera_follow_car,
            )
                .chain()
                .run_if(in_state(SimulationState::Running)));
    }
}

#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Turn(Scalar),
}

// TODO: flip CarDimensions and CarProperties to Components
struct CarDimensions {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

struct CarProperties {
    pub dimensions: CarDimensions,
    pub starting_pos: Transform,
    pub float_height: f32,
    pub float_bob_height: f32,
}

impl Default for CarProperties {
    fn default() -> Self {
        return Self {
            dimensions: CarDimensions { length: 2.5, width: 1.5, height: 0.75 },
            float_height: 1.0,
            float_bob_height: 0.25,
            starting_pos: Transform::from_xyz(0.0, 0.5, 0.0),
        };
    }
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
}

#[derive(Component)]
pub struct MovementAcceleration {
    linear: Scalar,
    angular: Scalar,
}

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct FloatImpulse(Scalar);

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    float_impulse: FloatImpulse,
}

impl MovementBundle {
    pub const fn new(
        linear_acceleration: Scalar,
        angular_acceleration: Scalar,
        damping: Scalar,
        float_impulse: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration { linear: linear_acceleration, angular: angular_acceleration },
            damping: MovementDampingFactor(damping),
            float_impulse: FloatImpulse(float_impulse),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 20.0, 0.9, 10.0)
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
        }
    }

    pub fn with_movement(
        mut self,
        linear_acceleration: Scalar,
        angular_acceleration: Scalar,
        damping: Scalar,
        float_impulse: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(linear_acceleration, angular_acceleration, damping, float_impulse);
        self
    }
}

fn setup_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let props = CarProperties::default();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(props.dimensions.width, props.dimensions.height, props.dimensions.length)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: props.starting_pos,
            ..default()
        },
        CarControllerBundle::new(Collider::cuboid(props.dimensions.width, props.dimensions.height, props.dimensions.length))
            .with_movement(30.0, 20.0, 0.92, 20.0),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
    ));
}

fn camera_follow_car(
    q_car: Query<&Transform, With<CarController>>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<CarController>)>,
) {
    if let Ok(car_transform) = q_car.get_single() {
        if let Ok(mut camera_transform) = q_camera.get_single_mut() {
            let car_position = car_transform.translation;
            let car_forward = car_transform.forward();

            // Camera should follow the car from above and slightly behind it
            let follow_distance = 15.0;
            let follow_height = 10.0;

            // Calculate desired camera position behind the car
            let mut desired_camera_position = car_position - car_forward * follow_distance;
            desired_camera_position.y += follow_height;

            // Smoothly move the camera to the desired position
            camera_transform.translation = desired_camera_position;

            // Make the camera look at the car with a slight downward angle
            camera_transform.look_at(car_position, Vec3::Y);
        }
    }
}

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
    q_car_transform: Query<&Transform, With<CarController>>,
) {
    let car_transform = q_car_transform.single();
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

fn make_car_float(
    time: Res<Time>,
    mut controllers: Query<(
        &FloatImpulse,
        &mut LinearVelocity,
    )>,
    q_car_transform: Query<&Transform, With<CarController>>,
    spatial_query: SpatialQuery
) {
    let car_transform = q_car_transform.single();
    let car_props = CarProperties::default();

    if let Some(hit) = spatial_query.cast_ray(
        car_transform.translation - (0.01 + Vec3::Y * car_props.dimensions.height / 2.0), // 0.01 puts the tracer just outside the chassis of the car, guaranteeing no clipping.
        Dir3::NEG_Y,
        car_props.float_bob_height + car_props.float_height,
        true,
        SpatialQueryFilter::default(),
    ) {
        for (float_impulse, mut linear_velocity) in &mut controllers
        {
            let proportional_to_distance = 1.0 - sigmoid((hit.time_of_impact - car_props.float_height) / car_props.float_bob_height);
            println!("Distance: {:?} -- Urgency: {:?}", hit.time_of_impact, proportional_to_distance);
            linear_velocity.y += proportional_to_distance * float_impulse.0 * time.delta_seconds();
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