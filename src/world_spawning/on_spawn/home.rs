use crate::*;
use avian3d::{collision::Sensor, prelude::Collider};
use bevy::prelude::*;

use self::home::Home;

pub(super) fn spawn(
    q_home: Query<&Children, Added<Home>>,
    mut q_child: Query<(&Handle<Mesh>, &mut Visibility)>,
    meshes: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    for children in q_home.iter() {
        for child_entity in children.iter() {
            let (mesh_handle, mut visibility) = q_child.get_mut(*child_entity).unwrap();
            let mesh = meshes.get(mesh_handle).unwrap();

            commands.entity(*child_entity).insert((
                Home,
                Collider::trimesh_from_mesh(&mesh).unwrap(),
                Sensor,
            ));

            *visibility = Visibility::Hidden;
        }
    }
}
