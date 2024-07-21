use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimulationState>()
            .add_systems(Startup, setup_simulation)
            .add_systems(Update, toggle_simulation)
            .add_systems(OnEnter(SimulationState::Paused), on_simulation_paused)
            .add_systems(OnExit(SimulationState::Paused), on_simulation_unpaused);
    }
}

fn toggle_simulation(
    keys: Res<ButtonInput<KeyCode>>,
    curr_simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_simulation_state.set(match curr_simulation_state.get() {
            SimulationState::Running => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Running,
        });
    }
}

fn setup_simulation(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single_mut();
    grab_cursor(window);
}

fn on_simulation_paused(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // TODO: It would be great to add some kind of text that says we're paused.
    // Unfortunately, I've no idea how to implement some overlay/text in front of the camera at this time.
    println!("Game paused.");

    let window = q_windows.single_mut();
    release_cursor(window);
}

fn on_simulation_unpaused(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // TODO: It would be great to remove said text that says we're paused.
    println!("Game unpaused.");

    let window = q_windows.single_mut();
    grab_cursor(window);
}

fn release_cursor(mut window: Mut<Window>) {
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn grab_cursor(mut window: Mut<Window>) {
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}
