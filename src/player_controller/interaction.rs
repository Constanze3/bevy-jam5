use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use super::Player;

pub fn plugin(app: &mut App) {
    app.add_event::<PickUpEvent>()
        .add_systems(Update, (interact).before(pick_up));
}

#[derive(Component)]
pub struct Hand;

#[derive(Component)]
pub struct UpPickable;

#[derive(Event)]
struct PickUpEvent(Entity);

fn interact(
    keys: Res<ButtonInput<KeyCode>>,
    query: SpatialQuery,
    q_camera: Query<&Transform, With<Camera>>,
    q_entities: Query<
        (Entity, Option<&Player>, Option<&Name>, Option<&UpPickable>),
        Without<Camera>,
    >,
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

        let (entity, _, name, up_pickable) = q_entities.get(hit.entity).unwrap();

        if let Some(n) = name {
            println!("{}", n.as_str())
        }

        println!("{:?}", up_pickable.is_none());

        if up_pickable.is_some() {
            pick_up_ew.send(PickUpEvent(entity));
        }
    }
}

fn pick_up(
    mut pick_up_er: EventReader<PickUpEvent>,
    mut q_up_pickable: Query<&mut Transform, With<UpPickable>>,
    q_hand: Query<(Entity, &Transform), With<Hand>>,
    mut commands: Commands,
) {
    for ev in pick_up_er.read() {
        println!("runs");
        let entity = ev.0;
        let mut transform = q_up_pickable.get_mut(entity).unwrap();

        let (hand_entity, hand_transform) = q_hand.get_single().unwrap();

        commands.entity(entity).set_parent(hand_entity);
        // transform.translation = hand_transform.translation;
    }
}
