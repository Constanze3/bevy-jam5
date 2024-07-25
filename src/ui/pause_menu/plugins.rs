use bevy::prelude::*;

use super::{systems::*, ShowPauseMenu, ShowSensitivityMenu};
use crate::simulation_state::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowSensitivityMenu>();
        app.add_event::<ShowPauseMenu>();
        app.add_systems(OnEnter(SimulationState::Paused), show_pause_menu)
            .add_systems(OnExit(SimulationState::Paused), hide_pause_menu)
            .add_systems(Update, interact_with_buttons)
            .add_systems(Update, interact_with_sensitivity_buttons)
            .add_systems(Update, update_sensitivity)
            .add_systems(Update, handle_show_pause_menu)
            .add_systems(Update, show_sensitivity_menu);
    }
}
