use super::*;
use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::player_car_swap::Rider;

use interaction;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(interaction::plugin)
            //.add_event::<MovementAction>()
            .insert_resource(PlayerControls::default())
            .insert_resource(PlayerSettings::default())
            .add_systems(
                Update,
                (
                    //keyboard_input,
                    //gamepad_input,
                    update_grounded,
                    apply_gravity,
                    player_look,
                    movement,
                    apply_movement_damping,
                )
                    .chain(),
            )
            .add_systems(
                // Run collision handling after collision detection.
                //
                // NOTE: The collision implementation here is very basic and a bit buggy.
                //       A collide-and-slide algorithm would likely work better.
                PostProcessCollisions,
                kinematic_controller_collisions,
            );
        app.add_systems(Update, connect_camera_to_reciever);
    }
}

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    collision_layers: CollisionLayers,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
    player_marker: Player,
    player_name: Name,
    desired_direction: DesiredDirection,
    rider: Rider,
    locked_axes: LockedAxes,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

#[derive(PhysicsLayer)]
pub enum CollisionMask {
    Player,
    Car,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            collider,
            collision_layers: CollisionLayers::new(CollisionMask::Player, CollisionMask::Car),
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
            player_marker: Player,
            desired_direction: DesiredDirection::default(),
            player_name: Name::new("player"),
            rider: Rider {
                ride: None,
                bottom_pos: Vec3::default(),
            },
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}
