use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    ecs::system::EntityCommands,
    gltf::{GltfMesh, GltfNode},
    prelude::*,
};

use crate::*;
use on_spawn::*;

pub mod on_spawn;

use self::{
    asset_loading::GltfAssets,
    home::Home,
    player_controller::{pick_up::UpPickable, Player},
};

// Marker components can be attached with the SpawnHook based on a function that is provided with the
// name of the object.
//
// Each object is an empty entity with a SpatialBundle that has one or more children (primitives)
// that contain meshes and materials.
//
// The loading of associated data can be done by querying the marker components.

pub struct SpawnWorldPlugin;

impl Plugin for SpawnWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(on_spawn::plugin)
            .add_systems(
                OnEnter(GameState::Spawning),
                (spawn_world, spawn_after_world).chain(),
            )
            .init_resource::<SpawnHook>();
    }
}

type Hook = Box<dyn Fn(&str, &mut EntityCommands) + Send + Sync + 'static>;

#[derive(Resource)]
pub struct SpawnHook(Hook);

impl Default for SpawnHook {
    fn default() -> Self {
        let hook: Hook = Box::new(|name, commands| {
            let class = name.split('.').next().unwrap_or(name);

            for keyword in class.split(' ') {
                match keyword {
                    "Bicycle" => {
                        commands.insert(Bicycle);
                    }
                    "Illegal" => {
                        commands.insert(Illegal);
                    }
                    "Player" => {
                        commands.insert(Player);
                    }
                    "Car" => {
                        commands.insert(Car);
                    }
                    "Home" => {
                        commands.insert(Home);
                    }
                    _ => {
                        commands.insert(MapElement);
                    }
                }
            }
        });

        return Self(hook);
    }
}

pub fn spawn_world(
    gltf_assets: Res<GltfAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut commands: Commands,
    spawn_hook: Res<SpawnHook>,
) {
    let world = gltfs.get(&gltf_assets.world).unwrap();

    for (name, gltf_node) in world
        .named_nodes
        .iter()
        .map(|(k, v)| (k.clone(), gltf_nodes.get(v).unwrap()))
    {
        let gltf_mesh = if let Some(handle) = &gltf_node.mesh {
            gltf_meshes.get(handle).unwrap()
        } else {
            println!("{} - empty node", name);
            continue;
        };

        let entity = commands
            .spawn((
                Name::new(name.to_string()),
                SpatialBundle {
                    transform: gltf_node.transform,
                    ..default()
                },
            ))
            .id();

        for primitive in gltf_mesh.primitives.iter() {
            let material = if let Some(it) = &primitive.material {
                it.clone()
            } else {
                Handle::<StandardMaterial>::default()
            };

            let primitive_entity = commands
                .spawn(PbrBundle {
                    mesh: primitive.mesh.clone(),
                    material,
                    ..default()
                })
                .id();

            commands.entity(entity).add_child(primitive_entity);
        }

        spawn_hook.0(&*name, &mut commands.entity(entity));
    }
}

pub fn spawn_after_world(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(224, 208, 208),
        brightness: 600.0,
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

    // a cube to move around
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(3.0, 2.0, 3.0),
                ..default()
            },
            RigidBody::Dynamic,
            UpPickable,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::default()),
                    material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                    ..default()
                },
                Collider::cuboid(1.0, 1.0, 1.0),
            ));
        });

    next_state.set(GameState::Playing);
}
