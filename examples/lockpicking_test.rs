use avian3d::{
    collision::Collider,
    debug_render::PhysicsDebugPlugin,
    math::{Scalar, Vector},
    prelude::RigidBody,
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_camera_extras::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_jam5::lockpicking::*;
use bevy_jam5::player_controller::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(LockPickingPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(CharacterControllerPlugin)
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            keybinds_override: Some(KeyBindings {
                // to disable switching keybindings, how about we just set it to a key the user wont(probably) have access to?
                switch_camera_mode: KeyCode::NonConvert,
                ..default()
            }),
            movement_settings_override: None,
        })
        .add_plugins(WorldInspectorPlugin::default())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut q_window: Query<&mut Window>,
) {
    // circular base
    let plane_mesh = Mesh::from(Cuboid::new(20.0, 20.0, 1.0));
    let plane_mesh_handle = meshes.add(plane_mesh.clone());

    commands.spawn((
        PbrBundle {
            mesh: plane_mesh_handle,
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(20.0, 20.0, 1.0),
        //Collider::convex_decomposition_from_mesh(&plane_mesh).unwrap(),
        Name::new("base_plate"),
    ));

    // pick target box
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 1.0, -5.0),
            material: materials.add(Color::LinearRgba(LinearRgba::BLUE)),
            ..default()
        },
        Locked {
            success_zone_width: 10.0,
            move_on_good_pick: true,
            zone_slide_settings: SlideSettings::SlideLinear(SlideLinear {
                speed: 10.0,
                time_to_target: 1.5,
            }),
        },
        Name::new("treasure box"),
    ));

    // Player
    let player = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..default()
            },
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
            LockPicker { target: None },
        ))
        .id();

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraControls {
            camera_mode: bevy_camera_extras::CameraMode::FirstPerson,
            attach_to: player,
        },
    ));
    // set camera to follow player

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
