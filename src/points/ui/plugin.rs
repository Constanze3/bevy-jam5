use crate::GameState;
use bevy::prelude::*;

use super::systems::*;

pub struct PointsUIPlugin;

impl Plugin for PointsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_points_ui)
            .add_systems(PostUpdate, update_points_ui);
    }
}
