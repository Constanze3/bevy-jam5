use bevy::prelude::*;

use crate::GameState;

use super::resources::*;
use super::systems::*;
use super::ui::*;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PointsAction>()
            .add_plugins(PointsUIPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (
                keyboard_input,
                handle_events,
            ).run_if(in_state(GameState::Playing)));
    }
}
