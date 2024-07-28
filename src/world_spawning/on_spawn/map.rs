use crate::*;
use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map(pub Entity);

#[derive(Component)]
pub struct MapElement;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn)
        .add_systems(Update, spawn_element.run_if(in_state(GameState::Spawning)));
}

fn spawn(mut commands: Commands) {
    let map_entity = commands
        .spawn((
            Name::new("Map"),
            SpatialBundle::default(),
            RigidBody::Static,
        ))
        .id();

    commands.insert_resource(Map(map_entity));
}

fn spawn_element(
    map: Res<Map>,
    q_map_element: Query<(Entity, &Children), Added<MapElement>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, children) in q_map_element.iter() {
        commands.entity(entity).set_parent(map.0);

        // generate colliders for children
        for child in children.iter() {
            let mesh = meshes.get(mesh_handles.get(*child).unwrap()).unwrap();
            commands
                .entity(*child)
                .insert(Collider::trimesh_from_mesh(mesh).unwrap());
        }
    }
}
