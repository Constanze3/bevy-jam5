use bevy::prelude::*;

use crate::{simulation_state::SimulationState, GameState};

use super::resources::*;
use super::systems::*;
use super::ui::*;

pub struct CarControllerPlugin;

impl Plugin for CarControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .add_plugins(CarUIPlugin)
            .add_systems(
                Update,
                (
                    keyboard_input.run_if(in_state(SimulationState::Running)),
                    movement.run_if(in_state(SimulationState::Running)),
                    decrement_fuel.run_if(in_state(SimulationState::Running)),
                    apply_movement_damping,
                    make_car_float,
                    stick_bicycles,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing).and_then(car_exists)),
            );
    }
}
