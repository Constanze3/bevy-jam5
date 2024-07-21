use bevy::prelude::*;

pub fn to_vec(dir: Dir3) -> Vec3 {
    return Vec3{ x: dir.x, y: dir.y, z: dir.z };
}
