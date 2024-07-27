use bevy::prelude::*;

use super::*;

pub struct PlayerCarSwapPlugin;

impl Plugin for PlayerCarSwapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, enter_car)
        ;
    }
}