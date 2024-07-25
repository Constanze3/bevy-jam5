mod cameras;
mod car_controller;
mod cubemap_factory;
mod plugin;
mod resources;
mod simulation_state;
mod ui;
mod utils;

use avian3d::{math::*, prelude::*};
use bevy::{core_pipeline::Skybox, prelude::*};
use bevy_flycam::prelude::PlayerPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cameras::*;
use car_controller::*;
use cubemap_factory::*;
use plugin::CharacterControllerPlugin;
use resources::*;
use simulation_state::*;
use ui::pause_menu::plugins::PauseMenuPlugin;

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .init_resource::<MovementSettings>()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
            // TODO: fix PauseMenuPlugin
            // PauseMenuPlugin,
            SimulationStatePlugin,
            WorldInspectorPlugin::new(),
            // PlayerPlugin,
            CarControllerPlugin,
            CubemapFactoryPlugin,
        ))
        .add_systems(Startup, (setup_world, setup_camera).chain())
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

    commands
        .entity(cameras.get_single().unwrap())
        .insert(Skybox {
            image: cubemap_factory.load_from_folder("sky", assets, images),
            brightness: 1000.0,
        });

    next_state.set(TestSkyboxState::Done);
}

fn setup_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: assets.load("town.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(182, 205, 214),
        brightness: 500.0,
    });

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
