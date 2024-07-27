use bevy::prelude::States;

pub mod asset_loading;
pub mod world_spawning;
pub mod lockpicking;

pub mod player_controller;
pub mod player_car_swap;
pub mod car_controller;
pub mod simulation_state;

pub mod cubemap_factory;
pub mod resources;
pub mod utils;


#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}