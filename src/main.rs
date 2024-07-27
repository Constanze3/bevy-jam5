use asset_loading::AssetLoaderPlugin;
use avian3d::{math::*, prelude::*};
use bevy::{
    core_pipeline::{prepass::NormalPrepass, Skybox},
    prelude::*,
};
use bevy_camera_extras::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam5::player_car_swap::*;
use bevy_jam5::player_controller::*;
use bevy_jam5::simulation_state::*;
use bevy_jam5::{asset_loading, cubemap_factory::*, world_spawning::*, *};
use bevy_jam5::{car_controller::*, player_controller::pick_up::UpPickable};

//use plugin::*;
use bevy_outline_post_process::{components::OutlinePostProcessSettings, OutlinePostProcessPlugin};

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            WorldInspectorPlugin::new(),
            // PlayerPlugin,
            CarControllerPlugin,
            CubemapFactoryPlugin,
            PlayerCarSwapPlugin,
            CharacterControllerPlugin,
            OutlinePostProcessPlugin,
            AssetLoaderPlugin,
            SpawnWorldPlugin,
            CameraExtrasPlugin {
                cursor_grabbed_by_default: true,
                keybinds_override: Some(KeyBindings {
                    // to disable switching keybindings, how about we just set it to a key the user wont(probably) have access to?
                    switch_camera_mode: KeyCode::NonConvert,
                    ..default()
                }),
                movement_settings_override: None,
            },
        ))
        .init_state::<GameState>()
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup_world)
        .init_state::<TestSkyboxState>()
        .add_systems(
            Update,
            test_skybox.run_if(in_state(TestSkyboxState::Waiting)),
        )
        .insert_resource(MovementSettings::default())
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
    // // Player
    // let player = commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Capsule3d::new(0.4, 1.0)),
    //         material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
    //         transform: Transform::from_xyz(0.0, 1.5, 0.0),
    //         ..default()
    //     },
    //     CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
    //         .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    // )).id();

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
