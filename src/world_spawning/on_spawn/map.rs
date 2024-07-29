use avian3d::{
    collision::CollisionMargin,
    dynamics::ccd::SweptCcd,
    prelude::{Collider, RigidBody},
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map(pub Entity);

#[derive(Component)]
pub struct MapElement;

pub(super) fn spawn(mut commands: Commands) {
    let map_entity = commands
        .spawn((
            Name::new("Map"),
            SpatialBundle::default(),
            RigidBody::Static,
            SweptCcd::default(),
        ))
        .id();

    commands.insert_resource(Map(map_entity));
}

pub(super) fn spawn_element(
    map: Res<Map>,
    q_map_element: Query<(Entity, &Children), Added<MapElement>>,
    q_child: Query<&Handle<Mesh>>,
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, children) in q_map_element.iter() {
        commands.entity(entity).set_parent(map.0);

        // generate colliders for children
        for child in children.iter() {
            let mesh = meshes.get(q_child.get(*child).unwrap()).unwrap();
            commands.entity(*child).insert((
                Collider::trimesh_from_mesh(mesh).unwrap(),
                CollisionMargin(0.05),
            ));
        }
    }
}
