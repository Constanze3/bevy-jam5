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
mod car_controller;
mod cameras;
mod resources;

use avian3d::{math::*, prelude::*};
use bevy::prelude::*;
// use bevy_flycam::prelude::*;

use car_controller::*;
use plugin::*;
use resources::*;
use simulation_state::*;
use cameras::*;

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            CharacterControllerPlugin,
            CarControllerPlugin,
            // PlayerPlugin,
        ))
        .add_systems(Startup, (
            setup_world,
            setup_camera,
        ).chain())
        .run();
}

fn setup_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Environment (see the `collider_constructors` example for creating colliders from scenes)
    commands.spawn((
        SceneBundle {
            scene: assets.load("town.glb#Scene0"),
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
            scene: assets.load("town.glb#Scene0"),
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
