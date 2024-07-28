use bevy::prelude::*;

use super::systems::*;

pub struct CarUIPlugin;

impl Plugin for CarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fuel_ui)
            .add_systems(PostUpdate, update_fuel_ui);
    }
}
