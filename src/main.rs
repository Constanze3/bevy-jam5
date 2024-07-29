// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use asset_loading::AssetLoaderPlugin;
use avian3d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::{prepass::NormalPrepass, Skybox},
    prelude::*,
};
use bevy_camera_extras::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_jam5::player_car_swap::*;
use bevy_jam5::player_controller::*;
use bevy_jam5::points::*;
use bevy_jam5::simulation_state::*;
use bevy_jam5::{asset_loading, cubemap_factory::*, world_spawning::*, *};
use bevy_jam5::{car_controller::*, lockpicking::LockpickingPlugin};

use bevy_outline_post_process::{components::OutlinePostProcessSettings, OutlinePostProcessPlugin};

fn main() {
    App::new()
        .insert_resource(MovementSettings::default())
        .add_plugins((
            introduction::plugin,
            rules::plugin,
            pause_menu::plugin,
            home::plugin,
        ))
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default(),
            SimulationStatePlugin,
            // WorldInspectorPlugin::new(),
            CarControllerPlugin,
            CubemapFactoryPlugin,
            PlayerCarSwapPlugin,
            CharacterControllerPlugin,
            PointsPlugin,
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
            LockpickingPlugin,
            // PhysicsDebugPlugin::default(),
        ))
        .insert_resource(SubstepCount(50))
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
