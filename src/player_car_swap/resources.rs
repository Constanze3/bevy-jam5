use bevy::prelude::*;

#[derive(Event)]
pub enum RideAction {
    Mount(Entity),
    Dismount,
}