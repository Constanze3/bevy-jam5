use bevy::prelude::*;

use crate::GameState;

use super::systems::*;

pub struct CarMountingUIPlugin;

impl Plugin for CarMountingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_car_riding_ui)
            .add_systems(PostUpdate,
                update_car_riding_ui.run_if(in_state(GameState::Playing)));
    }
}
