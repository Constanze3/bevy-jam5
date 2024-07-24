use bevy::prelude::*;

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