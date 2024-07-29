use bevy::prelude::*;

#[derive(Event)]
pub enum PointsAction {
    Reset,
    Increment(u32),
    Decrement(u32),
}
