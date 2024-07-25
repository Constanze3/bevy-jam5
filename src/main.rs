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
use car_controller::*;
use cameras::*;

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
            CharacterControllerPlugin,
            OutlinePostProcessPlugin,
            AssetLoaderPlugin,
            SpawnWorldPlugin,
            // PhysicsDebugPlugin::default(),
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
