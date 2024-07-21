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

mod player_controller;

use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_camera_extras::{components::{AttachedTo, FlyCam}, plugins::CameraExtrasPlugin};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player_controller::plugins::*;

//use plugin::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
        ))
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true
        })
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .insert_resource(MovementSettings::default())

        .add_plugins(WorldInspectorPlugin::default())

        .run();
}

fn close_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut q_window: Query<&mut Window>,
    assets: Res<AssetServer>,
) {

    // camera
    let camera = commands.spawn(
        (
            Camera3dBundle {
                transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            FlyCam,
        )
    ).id();

    // Player
    let player = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0, camera)
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    )).id();

    // set camera to follow player
    commands.entity(camera).insert(AttachedTo(player));

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

// #[derive(Component)]
// struct MainCamera;

#[derive(Resource)]
struct MovementSettings {
    sensitivity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        return Self { sensitivity: 0.1 };
    }
}

// fn camera_control(
//     //mut q_camera: Query<&mut Transform, With<MainCamera>>,
//     mut evr_mouse_motion: EventReader<MouseMotion>,
//     movement_settings: Res<MovementSettings>,
// ) {
//     let sensitivity = movement_settings.sensitivity;

//     let mut transform = q_camera.get_single_mut().unwrap();
//     for ev in evr_mouse_motion.read() {
//         let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

//         yaw -= (ev.delta.x * sensitivity).to_radians();
//         pitch -= (ev.delta.y * sensitivity).to_radians();

//         pitch = pitch.clamp(-1.54, 1.54);

//         transform.rotation =
//             Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
//     }
// }
