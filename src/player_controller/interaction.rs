use crate::damping::SmoothDamp;
use avian3d::{
    collision::Collider,
    dynamics::rigid_body::{ExternalForce, RigidBody},
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::prelude::*;

use super::Player;

pub fn plugin(app: &mut App) {
    app.init_resource::<ThrowForce>()
        .register_type::<ThrowForce>()
        .add_event::<PickUpEvent>()
        .add_systems(Update, ((interact, pick_up).chain(), throw))
        .add_systems(PostUpdate, move_fake_hand);
}

#[derive(Component)]
pub struct UpPickable;

#[derive(Event)]
struct PickUpEvent(Entity);

#[derive(Component, Default)]
pub struct Hand(Option<Entity>);

#[derive(Component)]
pub struct FakeHand;

fn move_fake_hand(
    q_camera: Query<&Transform, With<Camera>>,
    mut q_fake_hand: Query<&mut Transform, (With<FakeHand>, Without<Camera>)>,
) {
    let camera_transform = q_camera.get_single().unwrap();

    for mut transform in q_fake_hand.iter_mut() {
        let cf = camera_transform.forward();
        let forward = Vec3::new(cf.x, 0.0, cf.z).normalize();

        let target_translation = camera_transform.translation + forward * 1.7;
        transform.translation = Vec3::new(
            target_translation.x,
            camera_transform.translation.y - 0.2,
            target_translation.z,
        );

        let target_yaw = camera_transform.rotation.to_euler(EulerRot::YXZ).0;
        transform.rotation = Quat::from_rotation_y(target_yaw);
    }
}

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
            100.0,
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
    mut q_object: Query<(
        &mut RigidBody,
        &mut Visibility,
        &Handle<Mesh>,
        &Handle<StandardMaterial>,
        &Collider,
    )>,
    mut q_hand: Query<(Entity, &mut Hand)>,
    q_fake_hand: Query<Entity, With<FakeHand>>,
    mut commands: Commands,
) {
    let (hand_entity, mut hand) = q_hand.get_single_mut().unwrap();
    if hand.0.is_some() {
        return;
    }

    for ev in pick_up_er.read() {
        let entity = ev.0;

        let (mut rigidbody, mut visibility, mesh, material, collider) =
            q_object.get_mut(entity).unwrap();

        *rigidbody = RigidBody::Static;
        *visibility = Visibility::Hidden;

        hand.0 = Some(entity);

        commands.entity(hand_entity).with_children(|parent| {
            parent.spawn((collider.clone(),));
        });

        let fake_hand_entity = q_fake_hand.get_single().unwrap();
        commands.entity(fake_hand_entity).with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                ..default()
            });
        });

        return;
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ThrowForce(f32);

impl Default for ThrowForce {
    fn default() -> Self {
        return Self(600.0);
    }
}

fn throw(
    buttons: Res<ButtonInput<MouseButton>>,
    mut q_hand: Query<(Entity, &mut Hand)>,
    q_fake_hand: Query<(Entity, &Transform), With<FakeHand>>,
    mut q_object: Query<
        (
            &mut Transform,
            &mut RigidBody,
            &mut Visibility,
            &mut ExternalForce,
        ),
        (Without<FakeHand>, Without<Camera>),
    >,
    q_camera: Query<&Transform, (With<Camera>, Without<FakeHand>)>,
    throw_force: Res<ThrowForce>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (hand_entity, mut hand) = q_hand.get_single_mut().unwrap();
        if let Some(entity) = hand.0 {
            let (fake_hand_entity, fake_hand_transform) = q_fake_hand.get_single().unwrap();

            let (mut transform, mut rigidbody, mut visibility, mut external_force) =
                q_object.get_mut(entity).unwrap();

            commands.entity(hand_entity).despawn_descendants();
            commands.entity(fake_hand_entity).despawn_descendants();

            transform.translation = fake_hand_transform.translation;
            transform.rotation = fake_hand_transform.rotation;

            *visibility = Visibility::Inherited;
            *rigidbody = RigidBody::Dynamic;

            let camera_transform = q_camera.get_single().unwrap();

            external_force.set_force(camera_transform.forward() * throw_force.0);
            external_force.persistent = false;

            hand.0 = None;
        }
    }
}
