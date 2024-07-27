use avian3d::{
    collision::Collider,
    dynamics::{
        rigid_body::{AngularVelocity, ExternalForce, LinearVelocity, RigidBody},
        solver::joints::{FixedJoint, Joint},
    },
    math::Vector,
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::{math::VectorSpace, prelude::*};

use crate::damping::{smooth_damp, Pid, SmoothDamp, TransformPid};

use super::Player;

pub fn plugin(app: &mut App) {
    app.init_resource::<ThrowForce>()
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
    mut q_fake_hand: Query<(&mut Transform, &mut SmoothDamp), (With<FakeHand>, Without<Camera>)>,
    time: Res<Time>,
) {
    let camera_transform = q_camera.get_single().unwrap();

    for (mut transform, mut damp) in q_fake_hand.iter_mut() {
        let delta_time = time.delta_seconds();

        let cf = camera_transform.forward();
        let forward = Vec3::new(cf.x, 0.0, cf.z).normalize();

        let translation = transform.translation;
        let target_translation = camera_transform.translation + forward * 1.7;

        // damp.calculate(translation, target_translation, delta_time);

        transform.translation = Vec3::new(
            target_translation.x,
            camera_transform.translation.y - 0.2,
            target_translation.z,
        );

        let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
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
pub struct ThrowForce(f32);

impl Default for ThrowForce {
    fn default() -> Self {
        return Self(4.0);
    }
}

fn throw(
    buttons: Res<ButtonInput<MouseButton>>,
    // q_joint: Query<Entity, With<HandJoint>>,
    // mut q_child: Query<&mut ExternalForce>,
    // q_camera: Query<&Transform, With<Camera>>,
    // throw_force: Res<ThrowForce>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // for joint_entity in q_joint.iter() {
        //     commands.entity(joint_entity).despawn_recursive();
        // }
        // let Some(children) = q_hand.get_single().unwrap() else {
        //     return;
        // };

        // let child = children.first().unwrap();

        // commands
        //     .entity(*child)
        //     .remove_parent_in_place()
        //     .insert(RigidBody::Dynamic);

        // let mut external_force = q_child.get_mut(*child).unwrap();
        // let camera_transform = q_camera.get_single().unwrap();

        // let direction = camera_transform.forward();
        // let magnitude = throw_force.0;

        // external_force.apply_force(direction * magnitude);
        // external_force.persistent = false;
    }
}
