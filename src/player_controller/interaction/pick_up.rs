use avian3d::{
    collision::{Collider, Sensor},
    dynamics::rigid_body::{ExternalForce, RigidBody},
};
use bevy::{ecs::system::QueryLens, prelude::*};

use super::*;
use crate::player_controller::Player;

pub fn plugin(app: &mut App) {
    app.register_type::<HandConfig>()
        .init_resource::<HandConfig>()
        .init_resource::<Hand>()
        .add_event::<PickUpEvent>()
        .add_systems(Update, ((pick_up).after(interact), drop, throw))
        .add_systems(PostUpdate, move_visual);
}

#[derive(Component)]
pub struct UpPickable;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HandConfig {
    offset: Vec3,
    throw_force: f32,
}

impl Default for HandConfig {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 0.0, -1.5),
            throw_force: 600.0,
        }
    }
}

#[derive(Resource, PartialEq, Eq, Default)]
pub enum Hand {
    Some {
        entity: Entity,
        collider: Entity,
        visual: Entity,
    },
    #[default]
    Empty,
}

impl Hand {
    fn is_empty(&self) -> bool {
        *self == Hand::Empty
    }
}

#[derive(Event)]
pub struct PickUpEvent(pub Entity);

fn pick_up(
    mut pick_up_er: EventReader<PickUpEvent>,
    mut hand: ResMut<Hand>,
    mut q_object: Query<(
        &mut RigidBody,
        &mut Visibility,
        &Handle<Mesh>,
        &Handle<StandardMaterial>,
        &Collider,
    )>,
    q_player: Query<(Entity, &GlobalTransform), With<Player>>,
    q_camera: Query<&GlobalTransform, (With<Camera>, Without<Player>)>,
    config: Res<HandConfig>,
    mut commands: Commands,
) {
    if !hand.is_empty() {
        return;
    }

    for ev in pick_up_er.read() {
        let entity = ev.0;

        let (mut rigidbody, mut visibility, mesh, material, collider) =
            q_object.get_mut(entity).unwrap();

        *rigidbody = RigidBody::Static;
        *visibility = Visibility::Hidden;

        commands.entity(entity).insert(Sensor);

        let (player_entity, player_gtranform) = q_player.get_single().unwrap();
        let camera_gtransform = q_camera.get_single().unwrap();

        let height_offset = Vec3::new(
            0.0,
            camera_gtransform.translation().y - player_gtranform.translation().y,
            0.0,
        );

        let collider = commands
            .spawn((
                TransformBundle {
                    local: Transform::from_translation(height_offset + config.offset),
                    ..default()
                },
                collider.clone(),
            ))
            .id();

        commands.entity(player_entity).add_child(collider);

        let visual = commands
            .spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                ..default()
            })
            .id();

        *hand = Hand::Some {
            entity,
            collider,
            visual,
        };

        return;
    }
}

fn move_visual(
    mut q_visual: Query<&mut Transform, Without<Camera>>,
    q_camera: Query<&Transform, With<Camera>>,
    hand: Res<Hand>,
    config: Res<HandConfig>,
) {
    if let Hand::Some { visual, .. } = *hand {
        let mut transform = q_visual.get_mut(visual).unwrap();
        let camera_transform = q_camera.get_single().unwrap();

        let rotation = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);
        let rotated_offset = rotation.mul_vec3(config.offset);

        transform.translation = camera_transform.translation + rotated_offset;
        transform.rotation = rotation;
    }
}

fn release(
    mut hand: ResMut<Hand>,
    mut ql_object: QueryLens<(&mut Transform, Option<&mut RigidBody>, &mut Visibility)>,
    mut commands: Commands,
) -> Option<Entity> {
    let Hand::Some {
        entity,
        collider,
        visual,
    } = *hand
    else {
        return None;
    };

    let mut q_object = ql_object.query();

    let (visual_transform, _, _) = q_object.get(visual).unwrap();
    let visual_transform = visual_transform.clone();

    let (mut transform, rigidbody, mut visibility) = q_object.get_mut(entity).unwrap();
    let mut rigidbody = rigidbody.unwrap();

    transform.translation = visual_transform.translation;
    transform.rotation = visual_transform.rotation;

    commands.entity(collider).despawn_recursive();
    commands.entity(visual).despawn_recursive();

    *visibility = Visibility::Inherited;
    *rigidbody = RigidBody::Dynamic;

    commands.entity(entity).remove::<Sensor>();

    *hand = Hand::Empty;

    Some(entity)
}

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    hand: ResMut<Hand>,
    mut q_object: Query<(&mut Transform, Option<&mut RigidBody>, &mut Visibility)>,
    commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Right) {
        _ = release(hand, q_object.as_query_lens(), commands);
    }
}

fn throw(
    buttons: Res<ButtonInput<MouseButton>>,
    q_camera: Query<&Transform, With<Camera>>,
    config: Res<HandConfig>,
    hand: ResMut<Hand>,
    mut q_object: Query<
        (
            &mut Transform,
            Option<&mut RigidBody>,
            &mut Visibility,
            Option<&mut ExternalForce>,
        ),
        Without<Camera>,
    >,
    commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let released = release(
            hand,
            q_object.transmute_lens::<(&mut Transform, Option<&mut RigidBody>, &mut Visibility)>(),
            commands,
        );

        if let Some(entity) = released {
            let mut external_force = q_object.get_mut(entity).unwrap().3.unwrap();
            let camera_transform = q_camera.get_single().unwrap();

            external_force.set_force(camera_transform.forward() * config.throw_force);
            external_force.persistent = false;
        }
    }
}
