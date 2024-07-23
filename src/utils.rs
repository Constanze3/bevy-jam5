use bevy::prelude::*;

pub fn to_vec(dir: Dir3) -> Vec3 {
    return Vec3{ x: dir.x, y: dir.y, z: dir.z };
}

pub fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}
