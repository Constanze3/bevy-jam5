use bevy::prelude::States;

pub mod asset_loading;
pub mod world_spawning;

pub mod car_controller;
pub mod player_car_swap;
pub mod player_controller;
pub mod simulation_state;
pub mod points;

pub mod cubemap_factory;
pub mod resources;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}
