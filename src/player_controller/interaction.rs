use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;
use pick_up::*;

use super::Player;

pub mod pick_up;

pub fn plugin(app: &mut App) {
    app.add_plugins(pick_up::plugin)
        .add_systems(Update, interact);
}

fn interact(
    keys: Res<ButtonInput<KeyCode>>,
    query: SpatialQuery,
    q_camera: Query<&Transform, With<Camera>>,
    q_parent: Query<Option<&Parent>>,
    q_entities: Query<(Entity, Option<&Player>, Option<&UpPickable>), Without<Camera>>,
    mut pick_up_ew: EventWriter<PickUpEvent>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        let transform = q_camera.get_single().unwrap();

        let origin = transform.translation;
        let direction = transform.forward();

        let Some(hit) = query.cast_ray_predicate(
            origin,
            direction,
            5.0,
            true,
            SpatialQueryFilter::default(),
            &|entity| q_entities.get(entity).unwrap().1.is_none(),
        ) else {
            return;
        };

        let Some(parent) = q_parent.get(hit.entity).unwrap() else {
            return;
        };
        let parent_entity = parent.get();

        let (entity, _, up_pickable) = q_entities.get(parent_entity).unwrap();

        if up_pickable.is_some() {
            pick_up_ew.send(PickUpEvent(entity));
        }
    }
}