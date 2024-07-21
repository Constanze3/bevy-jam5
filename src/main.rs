//! A basic implementation of a character controller for a kinematic rigid body.
//!
//! This showcases the following:
//!
//! - Basic directional movement and jumping
//! - Support for both keyboard and gamepad input
//! - A configurable maximum slope angle
//! - Collision response for kinematic bodies
//! - Loading a platformer environment from a glTF
//!
//! The character controller logic is contained within the `plugin` module.
//!
//! For a dynamic character controller, see the `dynamic_character_3d` example.
//!
//! ## Warning
//!
//! Note that this is *not* intended to be a fully featured character controller,
//! and the collision logic is quite basic.
//!
//! For a better solution, consider implementing a "collide-and-slide" algorithm,
//! or use an existing third party character controller plugin like Bevy Tnua
//! (a dynamic character controller).

mod plugin;
mod simulation_state;
mod utils;

use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, math::VectorSpace, prelude::*};

use plugin::*;
use simulation_state::*;
use utils::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            // CharacterControllerPlugin,
        ))
        .add_systems(Startup, (
            setup_world,
            setup_camera,
            setup_car
        ).chain())
        .add_systems(Update, (
            // first_person_camera_control,
            camera_follow_car,
            move_car,
            // free_camera_control
        ).run_if(in_state(SimulationState::Running)))
        .insert_resource(MovementSettings::default())
        .run();
}

fn setup_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Environment (see the `collider_constructors` example for creating colliders from scenes)
    commands.spawn((
        SceneBundle {
            scene: assets.load("character_controller_demo.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });
}

fn setup_camera(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0),
            ..default()
        },
    ));
}

#[derive(Component)]
struct Car;

fn setup_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    commands.spawn((
        Car,
        RigidBody::Dynamic,
        Collider::cuboid(1.5, 1.0, 3.0),
        CenterOfMass(Vec3 { x: 0.0, y: -0.25, z: 0.0 }),
        Mass(2000.0),
        ExternalForce::ZERO,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.5, 1.0, 3.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
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
            let follow_distance = 15.0;
            let follow_height = 20.0;

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
    time: Res<Time>,
    movement_settings: Res<MovementSettings>,
) {
    // let (mut transform, mut global_transform) = q_car_transform.single_mut();
    let (mut linear_velocity, mut angular_velocity) = q_car_velocity.single_mut();
    let car_transform = q_car_transform.single();

    // Calculate forward direction of the car
    let forward = to_vec(car_transform.forward()); // Direction the cube is facing
    let right = to_vec(car_transform.right()); // Right direction of the cube
    
    // Determine movement direction based on input
    if keyboard_input.pressed(KeyCode::KeyW) {
        linear_velocity += forward * movement_settings.car_speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        linear_velocity -= forward * movement_settings.car_speed * time.delta_seconds();
    }

    // Determine rotational momentum
    if keyboard_input.pressed(KeyCode::KeyA) {
        angular_velocity.y += 1.0 * time.delta_seconds(); // Rotate counter-clockwise
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        angular_velocity.y += -1.0 * time.delta_seconds(); // Rotate clockwise
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


fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_window: Query<&mut Window>,
    assets: Res<AssetServer>,
) {
    // Hide cursor
    q_window.get_single_mut().unwrap().cursor.visible = false;

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    ));

    // A cube to move around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(3.0, 2.0, 3.0),
            ..default()
        },
    ));

    // Environment (see the `collider_constructors` example for creating colliders from scenes)
    commands.spawn((
        SceneBundle {
            scene: assets.load("character_controller_demo.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });

    // Camera
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    ));
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct MovementSettings {
    sensitivity: f32,
    speed: f32,
    car_speed: f32
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { 
            sensitivity: 0.1,
            speed: 0.4,
            car_speed: 0.5,
        };
    }
}

fn first_person_camera_control(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    movement_settings: Res<MovementSettings>,
) {
    let sensitivity = movement_settings.sensitivity;

    let mut transform = q_camera.get_single_mut().unwrap();
    for ev in evr_mouse_motion.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= (ev.delta.x * sensitivity).to_radians();
        pitch -= (ev.delta.y * sensitivity).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}


fn free_camera_control(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
    movement_settings: Res<MovementSettings>,
) {
    let sensitivity = movement_settings.sensitivity;

    let mut transform = q_camera.get_single_mut().unwrap();
    for ev in evr_mouse_motion.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= (ev.delta.x * sensitivity).to_radians();
        pitch -= (ev.delta.y * sensitivity).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }

    let forward = transform.forward();
    let right = transform.right();
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += to_vec(forward);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= to_vec(forward);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= to_vec(right);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += to_vec(right);
    }
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * movement_settings.speed;
    }
}
