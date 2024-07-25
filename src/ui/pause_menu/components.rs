use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub enum ButtonAction {
    Resume,
    Quit,
    Sensitivity,
    Back,
}

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct SensitivityMenu;
