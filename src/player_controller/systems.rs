use avian3d::{math::*, prelude::*};
use bevy::{
    ecs::query::Has,
    input::mouse::MouseMotion,
    math::VectorSpace,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_camera_extras::*;

use crate::player_car_swap::Rider;

use super::*;

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
pub fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &Rider,
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        (With<RigidBody>, With<CharacterController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (rider, mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character_rb = *bodies.get(collider_parent1.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent2.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character_rb = *bodies.get(collider_parent2.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent1.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };
        // stop tracking collisions if palyer is riding soemthing
        if rider.ride.is_some() {
            continue;
        }

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.contacts.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_between(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points in the normal's direction in the XZ plane.
                    let normal_direction_xz =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_xz = linear_velocity.dot(normal_direction_xz);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_xz * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude = normal_speed
                    - (deepest_penetration / time.delta_seconds_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}

/// gives a marker component to target of camera so it can interop with its attached camera
pub fn connect_camera_to_reciever(
    mut commands: Commands,
    follower_cameras: Query<(Entity, &CameraControls)>, //marked_players: Query<Entity, (With<Player>, Without<BoundCamera>)>
) {
    for (camera, camera_controls) in follower_cameras.iter() {
        commands
            .entity(camera_controls.attach_to)
            .insert(BoundCamera(camera));
    }
}

/// Updates the [`Grounded`] status for character controllers.
pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Handles looking around if cursor is locked
pub fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<&mut Transform, (With<Camera>, With<CameraControls>)>,
) {
    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to rotate camera, Reason: {:#}", err);
            return;
        }
    };

    if restraints_toggled.0 == false {
        for mut transform in query.iter_mut() {
            //println!("making camera follow movement");

            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    player_controls: Res<PlayerControls>,
    player_settings: Res<PlayerSettings>,
    camera_locked_state_check: Option<Res<RestraintsToggled>>,
    //mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<
        (
            &MovementAcceleration,
            &JumpImpulse,
            &mut LinearVelocity,
            Has<Grounded>,
            &BoundCamera,
            &mut Transform,
            //&DesiredDirection,
        ),
        Without<Camera>,
    >,
    cameras: Query<(&Camera, &Transform), With<Camera>>,
) {
    match camera_locked_state_check {
        Some(camera_lock_state) => match camera_lock_state.0 {
            false => return,
            true => {}
        },
        None => {}
    }
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    //for event in movement_event_reader.read() {
    for (
        movement_acceleration,
        jump_impulse,
        mut linear_velocity,
        is_grounded,
        camera_entity,
        mut player_trans,
    ) in &mut controllers
    {
        let mut velocity = Vec3::ZERO;

        let axis = EulerRot::XYZ;
        let Ok((_, cam_trans)) = cameras.get(camera_entity.0) else {
            continue;
        };
        player_trans.rotation = Quat::from_rotation_y(cam_trans.rotation.to_euler(axis).1);

        player_trans.rotation = Quat::from_rotation_y(cam_trans.rotation.to_euler(EulerRot::YXZ).0);

        let local_z = player_trans.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        if keys.any_pressed(player_controls.forward.clone()) {
            velocity += forward;
        }
        if keys.any_pressed(player_controls.back.clone()) {
            velocity -= forward;
        }
        if keys.any_pressed(player_controls.left.clone()) {
            velocity -= right;
        }
        if keys.any_pressed(player_controls.right.clone()) {
            velocity += right;
        }
        if keys.pressed(KeyCode::Space) {
            velocity += Vec3::Y;
        }

        velocity = velocity.normalize_or_zero();

        let mut runspeed_increase = 1.0;

        if keys.any_pressed(player_controls.sprinting.clone()) {
            runspeed_increase = player_settings.run_speedup_factor;
        }
        linear_velocity.x = velocity.x
            * movement_acceleration.0
            * delta_time
            * player_settings.speed
            * runspeed_increase;
        linear_velocity.z = velocity.z
            * movement_acceleration.0
            * delta_time
            * player_settings.speed
            * runspeed_increase;

        if is_grounded {
            linear_velocity.y = velocity.y * jump_impulse.0;
        }
    }
}

/// Applies [`ControllerGravity`] to character controllers.
pub fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&ControllerGravity, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for (gravity, mut linear_velocity) in &mut controllers {
        linear_velocity.0 += gravity.0 * delta_time;
    }
}

/// Slows down movement in the XZ plane.
pub fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
