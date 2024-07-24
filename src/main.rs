mod asset_loading;
mod cameras;
mod car_controller;
mod cubemap_factory;
mod player_controller;
mod resources;
mod simulation_state;
mod utils;
mod world_spawning;

use asset_loading::AssetLoaderPlugin;
use avian3d::{math::*, prelude::*};
use bevy::{
    core_pipeline::{prepass::NormalPrepass, Skybox},
    prelude::*,
};
use bevy_camera_extras::{
    components::{AttachedTo, CameraControls},
    plugins::CameraExtrasPlugin,
};

use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_outline_post_process::{components::OutlinePostProcessSettings, OutlinePostProcessPlugin};
use player_controller::plugins::*;

use cubemap_factory::*;
use resources::*;
use simulation_state::*;
use world_spawning::SpawnWorldPlugin;
//use plugin::*;

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            WorldInspectorPlugin::new(),
            PlayerPlugin,
            CubemapFactoryPlugin,
            // CharacterControllerPlugin,
            OutlinePostProcessPlugin,
            AssetLoaderPlugin,
            SpawnWorldPlugin,
        ))
        .init_state::<GameState>()
        .insert_resource(Msaa::Off)
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
        //.add_systems(Update, close_on_esc)
        .insert_resource(MovementSettings::default())
        .run();
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
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
    commands.entity(cameras.get_single().unwrap()).insert((
        Skybox {
            image: cubemap_factory.load_from_folder("sky", assets, images),
            brightness: 1000.0,
        },
        NormalPrepass,
        OutlinePostProcessSettings::new(1.5, 0.0, false),
    ));

    next_state.set(TestSkyboxState::Done);
}

fn setup_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
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
        .id();

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraControls,
        AttachedTo(player),
    ));

    // a cube to move around
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

    // town
    commands.spawn((
        SceneBundle {
            scene: assets.load("town.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(182, 205, 214),
        brightness: 500.0,
    });

    // sunlight
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
