use bevy::prelude::*;

use super::*;
use super::ui::CarMountingUIPlugin;

pub struct PlayerCarSwapPlugin;

impl Plugin for PlayerCarSwapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(CarMountingUIPlugin)
        .add_systems(Update, enter_car)
        ;
    }
}