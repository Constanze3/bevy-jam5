use bevy::prelude::*;

mod bicycle;
mod car;
mod home;
mod map;
mod player;
mod trash;

pub use bicycle::*;
pub use car::*;
pub use map::*;
pub use trash::*;

use crate::GameState;

use super::spawn_world;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, map::spawn).add_systems(
        Update,
        (
            map::spawn_element,
            car::spawn,
            bicycle::spawn,
            home::spawn,
            player::spawn,
            trash::spawn,
        )
            .run_if(in_state(GameState::Spawning))
            .after(spawn_world),
    );
}
