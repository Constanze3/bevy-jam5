use bevy::prelude::*;

#[derive(Component)]
/// This entity can ride [`Ridable`]s
pub struct Rider {
    pub ride: Option<Entity>,
    pub bottom_pos: Vec3,
}

/// This entity can be riden by [`Rider`]s
#[derive(Component)]
pub struct Ridable {
    pub seat_offset: Transform,
}

