use bevy::prelude::*;

use crate::GameState;

use super::systems::*;

pub struct CarUIPlugin;

impl Plugin for CarUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_fuel_ui)
            .add_systems(
                PostUpdate,
                update_fuel_ui.run_if(in_state(GameState::Playing)),
            );
    }
}
