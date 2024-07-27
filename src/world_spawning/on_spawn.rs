use bevy::prelude::*;

mod bicycle;
mod car;
mod map;

pub use bicycle::*;
pub use car::*;
pub use map::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((map::plugin, car::plugin, bicycle::plugin));
}
