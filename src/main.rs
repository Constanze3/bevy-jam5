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

use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, math::VectorSpace, prelude::*};

use plugin::*;
use simulation_state::*;

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
        ))
        .add_systems(Update, (
            // first_person_camera_control,
            // camera_follow_car,
            free_camera_control
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

fn setup_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Car
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));
}

fn camera_follow_car(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    // let mut camera = q_camera.get_single_mut().unwrap();
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
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { sensitivity: 0.1, speed: 0.4 };
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
        direction.x += forward.x;
        direction.y += forward.y;
        direction.z += forward.z;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.x -= forward.x;
        direction.y -= forward.y;
        direction.z -= forward.z;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= right.x;
        direction.y -= right.y;
        direction.z -= right.z
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += right.x;
        direction.y += right.y;
        direction.z += right.z
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
