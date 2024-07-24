use avian3d::math::*;
use bevy::{ecs::prelude::*, prelude::{Deref, DerefMut, KeyCode}, reflect::Reflect};

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Reflect)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component, Reflect)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
pub struct MovementDampingFactor(pub Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct JumpImpulse(pub Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component, Reflect)]
pub struct ControllerGravity(pub Vector);

#[derive(Component, Clone, Copy, Deref, DerefMut, Default)]
pub struct DesiredDirection(pub Vector3);

#[derive(Resource, Reflect, Clone)]
pub struct PlayerControls {
    pub forward: Vec<KeyCode>,
    pub back: Vec<KeyCode>,
    pub left: Vec<KeyCode>,
    pub right: Vec<KeyCode>,

}

impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            forward: vec![KeyCode::KeyW, KeyCode::ArrowUp],
            back: vec![KeyCode::KeyS, KeyCode::ArrowDown],
            left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
            right: vec![KeyCode::KeyD, KeyCode::ArrowRight]
        }
    }
}


#[derive(Component, Reflect)]
pub struct Player;

/// the camera the player treats as "its" camera.
#[derive(Component)]
pub struct BoundCamera(pub Entity);