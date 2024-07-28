use bevy::prelude::States;

pub mod asset_loading;
pub mod world_spawning;

pub mod car_controller;
pub mod home;
pub mod player_car_swap;
pub mod player_controller;
pub mod points;
pub mod simulation_state;

pub mod cubemap_factory;
pub mod resources;
pub mod rules;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    Spawning,
    Playing,
}
