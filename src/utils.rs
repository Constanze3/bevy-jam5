use bevy::prelude::*;

pub fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}
