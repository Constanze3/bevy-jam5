use asset_loading::AssetLoaderPlugin;
use avian3d::{math::*, prelude::*};
use bevy::{
    core_pipeline::{prepass::NormalPrepass, Skybox},
    prelude::*,
};
use bevy_camera_extras::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam5::car_controller::*;
use bevy_jam5::player_car_swap::*;
use bevy_jam5::player_controller::*;
use bevy_jam5::simulation_state::*;
use bevy_jam5::{asset_loading, cubemap_factory::*, world_spawning::*, *};

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
            // PhysicsDebugPlugin::default(),
        ))
        .insert_resource(SubstepCount(500))
        .init_state::<GameState>()
        .insert_resource(Msaa::Off)
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
