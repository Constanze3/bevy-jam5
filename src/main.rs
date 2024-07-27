mod cameras;
mod car_controller;
mod cubemap_factory;
mod damping;
mod player_controller;
mod resources;
mod simulation_state;
mod utils;

use avian3d::{math::*, prelude::*};
use bevy::{core_pipeline::Skybox, prelude::*};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_camera_extras::CameraMode;
use bevy_camera_extras::{components::CameraControls, plugins::CameraExtrasPlugin};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use damping::{Pid, SmoothDamp, TransformPid};
use player_controller::plugins::*;
use player_controller::*;

use cubemap_factory::*;
use resources::*;
use simulation_state::*;
//use plugin::*;

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            WorldInspectorPlugin::new(),
            CubemapFactoryPlugin,
            CharacterControllerPlugin,
            damping::reflect_plugin,
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, (setup_world).chain())
        .init_state::<TestSkyboxState>()
        .add_systems(
            Update,
            test_skybox.run_if(in_state(TestSkyboxState::Waiting)),
        )
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            keybinds_override: None,
            movement_settings_override: None,
        })
        .insert_resource(MovementSettings::default())
        // .add_systems(FixedUpdate, camera_control)
        .run();
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum TestSkyboxState {
    #[default]
    Waiting,
    Done,
}

/// This system adds a skybox to the camera after it is loaded.
/// `TODO` properly initialize skybox instead.
fn test_skybox(
    cameras: Query<Entity, With<Camera>>,
    mut commands: Commands,
    mut cubemap_factory: ResMut<CubemapFactory>,
    assets: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<TestSkyboxState>>,
) {
    if cameras.is_empty() {
        return;
    }
    println!("camera count: {:#?}", cameras.iter().len());
    commands
        .entity(cameras.get_single().unwrap())
        .insert(Skybox {
            image: cubemap_factory.load_from_folder("sky", assets, images),
            brightness: 1000.0,
        });

    next_state.set(TestSkyboxState::Done);
}

fn setup_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Hand"),
                TransformBundle {
                    local: Transform::from_xyz(0.0, 1.0, -3.0),
                    ..default()
                },
                Hand::default(),
            ));
        })
        .id();

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraControls {
            attach_to: player,
            camera_mode: CameraMode::FirstPerson,
        },
    ));

    commands.spawn((
        Name::new("Fake Hand"),
        SpatialBundle::default(),
        FakeHand,
        SmoothDamp::new(3.0),
    ));

    // A cube to move around
    commands.spawn((
        UpPickable,
        Name::new("Cube"),
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
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(182, 205, 214),
        brightness: 500.0,
    });

    commands.insert_resource(AmbientLight::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 4.0, -0.7, 0.0),
            ..default()
        },
        ..default()
    });
}

fn camera_control(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
) {
    let sensitivity = 0.1;
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
