pub struct OnSpawnPlugin;

use bevy::prelude::*;

mod bicycle;
mod map;

pub use bicycle::*;
pub use map::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((bicycle::plugin, map::plugin));
}
