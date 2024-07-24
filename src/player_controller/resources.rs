use bevy::prelude::*;

#[derive(Resource, Reflect, Clone)]
pub struct PlayerControls {
    pub forward: Vec<KeyCode>,
    pub back: Vec<KeyCode>,
    pub left: Vec<KeyCode>,
    pub right: Vec<KeyCode>,
    pub sprinting: Vec<KeyCode>,
}

#[derive(Resource, Reflect, Clone, Copy)]
pub struct PlayerSettings {
    /// speed in ??? units
    pub speed: f32,
    /// how much faster player runs when running
    pub run_speedup_factor: f32
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            speed: 10.0,
            run_speedup_factor: 1.75
        }
    }
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            forward: vec![KeyCode::KeyW, KeyCode::ArrowUp],
            back: vec![KeyCode::KeyS, KeyCode::ArrowDown],
            left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
            right: vec![KeyCode::KeyD, KeyCode::ArrowRight],
            sprinting: vec![KeyCode::ShiftLeft],
        }
    }
}