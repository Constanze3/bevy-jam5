use bevy::prelude::*;

use crate::{GameState, simulation_state::SimulationState};

use super::systems::*;
use super::resources::*;

pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input.run_if(in_state(SimulationState::Running)),
                movement.run_if(in_state(SimulationState::Running)),
                apply_movement_damping,
                make_car_float,
            )
                .chain()
                .run_if(in_state(GameState::Playing).and_then(car_exists)),
        );
    }
}
