use avian3d::{
    dynamics::rigid_body::{AngularVelocity, ExternalForce, LinearVelocity, RigidBody},
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::{math::VectorSpace, prelude::*};

use super::Player;

pub fn plugin(app: &mut App) {
    app.init_resource::<ThrowForce>()
        .add_event::<PickUpEvent>()
        .add_systems(Update, ((interact, pick_up).chain(), throw));
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

        let (entity, _, up_pickable) = q_entities.get(hit.entity).unwrap();

        if up_pickable.is_some() {
            pick_up_ew.send(PickUpEvent(entity));
        }
    }
}

fn pick_up(
    mut pick_up_er: EventReader<PickUpEvent>,
    mut q_up_pickable: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<UpPickable>,
    >,
    q_hand: Query<(Entity, &Transform), (With<Hand>, Without<UpPickable>)>,
    mut commands: Commands,
) {
    for ev in pick_up_er.read() {
        let entity = ev.0;
        let (mut transform, mut linear_velocity, mut angular_velocity) =
            q_up_pickable.get_mut(entity).unwrap();

        linear_velocity.0 = Vec3::ZERO;
        angular_velocity.0 = Vec3::ZERO;

        let (hand_entity, hand_transform) = q_hand.get_single().unwrap();

        commands
            .entity(entity)
            .set_parent(hand_entity)
            .remove::<RigidBody>();
        transform.translation = hand_transform.translation;
    }
}

#[derive(Resource, Reflect)]
pub struct ThrowForce(f32);

impl Default for ThrowForce {
    fn default() -> Self {
        return Self(4.0);
    }
}

fn throw(
    buttons: Res<ButtonInput<MouseButton>>,
    q_hand: Query<Option<&Children>, With<Hand>>,
    mut q_child: Query<&mut ExternalForce>,
    q_camera: Query<&Transform, With<Camera>>,
    throw_force: Res<ThrowForce>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let Some(children) = q_hand.get_single().unwrap() else {
            return;
        };

        let child = children.first().unwrap();

        commands
            .entity(*child)
            .remove_parent_in_place()
            .insert(RigidBody::Dynamic);

        let mut external_force = q_child.get_mut(*child).unwrap();
        let camera_transform = q_camera.get_single().unwrap();

        let direction = camera_transform.forward();
        let magnitude = throw_force.0;

        // external_force.apply_force(direction * magnitude);
        // external_force.persistent = false;
    }
}
