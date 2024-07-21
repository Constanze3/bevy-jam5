use bevy::prelude::*;
use avian3d::prelude::*;

use super::{cameras::*, resources::*, simulation_state::*, utils::*};

#[derive(Component)]
struct Car;

pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_car)
            .add_systems(Update, (
                camera_follow_car,
                move_car,
            ).run_if(in_state(SimulationState::Running)));
    }
}

fn setup_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    commands.spawn((
        Car,
        RigidBody::Kinematic,
        Collider::cuboid(1.25, 1.0, 2.5),
        Mass(2000.0),
        ExternalForce::ZERO,
        LinearVelocity::ZERO,
        LinearDamping(10.0),
        AngularVelocity::ZERO,
        AngularDamping(10.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.25, 1.0, 2.5)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(100.0, 1.0, 100.0),
            ..default()
        },
    ));

    // Ensure the existing camera looks at the car
    if let Ok(mut camera_transform) = q_camera.get_single_mut() {
        let car_position = Vec3::new(0.0, 4.0, 0.0);
        let follow_distance = 15.0;
        let follow_height = 20.0;

        // Calculate desired camera position
        let mut desired_camera_position = car_position - Vec3::Z * follow_distance;
        desired_camera_position.y += follow_height;

        camera_transform.translation = desired_camera_position;
        camera_transform.look_at(car_position, Vec3::Y);
    }    
}

fn camera_follow_car(
    q_car: Query<&Transform, With<Car>>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Car>)>,
) {
    if let Ok(car_transform) = q_car.get_single() {
        if let Ok(mut camera_transform) = q_camera.get_single_mut() {
            let car_position = car_transform.translation;
            let car_forward = car_transform.forward();

            // Camera should follow the car from above and slightly behind it
            let follow_distance = 5.0;
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

fn move_car(
    mut q_car_velocity: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Car>>,
    q_car_transform: Query<&Transform, With<Car>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    movement_settings: Res<MovementSettings>,
) {
    // let (mut transform, mut global_transform) = q_car_transform.single_mut();
    let (mut linear_velocity, mut angular_velocity) = q_car_velocity.single_mut();
    let car_transform = q_car_transform.single();

    // Calculate forward direction of the car
    let forward = to_vec(car_transform.forward()); // Direction the cube is facing
    // let right = to_vec(car_transform.right()); // Right direction of the cube
    
    let forward_velocity = forward * movement_settings.car_drive_acceleration;

    // Determine movement direction based on input
    let mut accelerating = false;
    if keyboard_input.pressed(KeyCode::KeyW) {
        accelerating = true;
        linear_velocity.x = forward_velocity.x;
        linear_velocity.y = forward_velocity.y;
        linear_velocity.z = forward_velocity.z;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        accelerating = true;
        linear_velocity.x = -forward_velocity.x;
        linear_velocity.y = -forward_velocity.y;
        linear_velocity.z = -forward_velocity.z;
    }
    if !accelerating {
        linear_velocity.x = 0.0;
        linear_velocity.y = 0.0;
        linear_velocity.z = 0.0;
    }

    // Determine rotational momentum
    let mut turning = false;
    if keyboard_input.pressed(KeyCode::KeyA) {
        turning = true;
        angular_velocity.y = movement_settings.car_turn_acceleration; // Rotate counter-clockwise
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        turning = true;
        angular_velocity.y = -movement_settings.car_turn_acceleration; // Rotate clockwise
    }
    if !turning {
        angular_velocity.y = 0.0;
    }

    // // Move the car in the direction it is facing
    // if direction.length() > 0.0 {
    //     // Normalize the direction vector
    //     let normalized_direction = direction.normalize();
        
    //     // Calculate the movement vector based on the car's current orientation
    //     let move_vector = if normalized_direction.dot(forward) > 0.0 {
    //         forward * normalized_direction.dot(forward) + right * normalized_direction.dot(right)
    //     } else {
    //         -forward * normalized_direction.dot(forward) + -right * normalized_direction.dot(right)
    //     };

    //     transform.translation += move_vector * time.delta_seconds() * movement_settings.car_speed;
    // }
}
