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

use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_flycam::prelude::*;

use plugin::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
        ))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .insert_resource(MovementSettings::default())
        .run();
}

fn setup(
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
            scene: assets.load("test2.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    // Camera
    // commands.spawn((
    //     MainCamera,
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.0, 2.0, 0.0),
    //         ..default()
    //     },
    // ));
    //

    commands.insert_resource(AmbientLight::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct MovementSettings {
    sensitivity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { sensitivity: 0.1 };
    }
}

fn camera_control(
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
