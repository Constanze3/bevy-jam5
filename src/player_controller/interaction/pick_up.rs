use avian3d::{
    collision::{Collider, Sensor},
    dynamics::rigid_body::{ExternalForce, RigidBody},
};
use bevy::{ecs::system::QueryLens, prelude::*};

use super::*;
use crate::player_controller::{PickUpUIPlugin, Player};
pub fn plugin(app: &mut App) {
    app.register_type::<HandConfig>()
        .init_resource::<HandConfig>()
        .init_resource::<Hand>()
        .add_event::<PickUpEvent>()
        .add_plugins(PickUpUIPlugin)
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
            offset: Vec3::new(0.0, 0.0, -2.0),
            throw_force: 3000.0,
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
    pub fn is_empty(&self) -> bool {
        *self == Hand::Empty
    }
}

#[derive(Event)]
pub struct PickUpEvent(pub Entity);

fn pick_up(
    mut pick_up_er: EventReader<PickUpEvent>,
    mut hand: ResMut<Hand>,
    mut q_object: Query<(&mut Visibility, &mut RigidBody, &Children)>,
    q_child: Query<(
        &Transform,
        Option<&Handle<Mesh>>,
        Option<&Handle<StandardMaterial>>,
        Option<&Collider>,
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
        let (mut visibility, mut rigidbody, children) = q_object.get_mut(entity).unwrap();

        *visibility = Visibility::Hidden;
        *rigidbody = RigidBody::Static;

        let (player_entity, player_gtranform) = q_player.get_single().unwrap();
        let camera_gtransform = q_camera.get_single().unwrap();

        let height_offset = Vec3::new(
            0.0,
            camera_gtransform.translation().y - player_gtranform.translation().y,
            0.0,
        );

        let visual_parent = commands.spawn(SpatialBundle::default()).id();

        let collider_parent = commands
            .spawn(TransformBundle {
                local: Transform::from_translation(height_offset + config.offset),
                ..default()
            })
            .id();

        for child_entity in children.iter() {
            let (transform, mesh, material, collider) = q_child.get(*child_entity).unwrap();

            if let Some(collider) = collider {
                commands.entity(collider_parent).with_children(|parent| {
                    parent.spawn((
                        TransformBundle {
                            local: transform.clone(),
                            ..default()
                        },
                        collider.clone(),
                    ));
                });
            }

            if let Some(mesh) = mesh {
                if let Some(material) = material {
                    commands.entity(visual_parent).with_children(|parent| {
                        parent.spawn(PbrBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            ..default()
                        });
                    });
                }
            }

            commands.entity(*child_entity).insert(Sensor);
        }

        commands.entity(player_entity).add_child(collider_parent);

        *hand = Hand::Some {
            entity,
            collider: collider_parent,
            visual: visual_parent,
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
    mut ql_object: QueryLens<(
        &mut Transform,
        Option<&mut RigidBody>,
        &mut Visibility,
        &Children,
    )>,
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

    let (visual_transform, _, _, _) = q_object.get(visual).unwrap();
    let visual_transform = visual_transform.clone();

    let (mut transform, rigidbody, mut visibility, children) = q_object.get_mut(entity).unwrap();
    let mut rigidbody = rigidbody.unwrap();

    transform.translation = visual_transform.translation;
    transform.rotation = visual_transform.rotation;

    commands.entity(collider).despawn_recursive();
    commands.entity(visual).despawn_recursive();

    *visibility = Visibility::Inherited;
    *rigidbody = RigidBody::Dynamic;

    for child_entity in children {
        commands.entity(*child_entity).remove::<Sensor>();
    }

    *hand = Hand::Empty;

    Some(entity)
}

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    hand: ResMut<Hand>,
    mut q_object: Query<(
        &mut Transform,
        Option<&mut RigidBody>,
        &mut Visibility,
        &Children,
    )>,
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
            &Children,
            Option<&mut ExternalForce>,
        ),
        Without<Camera>,
    >,
    commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let released = release(
            hand,
            q_object.transmute_lens::<(
                &mut Transform,
                Option<&mut RigidBody>,
                &mut Visibility,
                &Children,
            )>(),
            commands,
        );

        if let Some(entity) = released {
            let mut external_force = q_object.get_mut(entity).unwrap().4.unwrap();
            let camera_transform = q_camera.get_single().unwrap();

            external_force.set_force(camera_transform.forward() * config.throw_force);
            external_force.persistent = false;
        }
    }
}
