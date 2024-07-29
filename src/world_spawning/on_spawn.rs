use bevy::prelude::*;

mod bicycle;
mod car;
mod home;
mod map;

pub use bicycle::*;
pub use car::*;
pub use map::*;

use crate::GameState;

use super::spawn_world;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, map::spawn).add_systems(
        Update,
        (map::spawn_element, car::spawn, bicycle::spawn, home::spawn)
            .run_if(in_state(GameState::Spawning))
            .after(spawn_world),
    );
}
