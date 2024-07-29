use bevy::prelude::*;

use crate::GameState;

use super::systems::*;

pub struct LockPickingUIPlugin;

impl Plugin for LockPickingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ui)
            .add_systems(PostUpdate, update_ui.run_if(in_state(GameState::Playing)));
    }
}
