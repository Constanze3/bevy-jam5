use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::resources::MenuAction;
use crate::pause_menu::PauseMenuUi;
use crate::rules::RulesUi;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
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

fn setup_simulation(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let window = q_windows.single_mut();
    grab_cursor(window);
}

fn on_simulation_paused(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut pause_event_writer: EventWriter<MenuAction<PauseMenuUi>>,
    mut rules_event_writer: EventWriter<MenuAction<RulesUi>>,
) {
    let window = q_windows.single_mut();
    release_cursor(window);
    pause_event_writer.send(MenuAction::Show);
    rules_event_writer.send(MenuAction::Hide);
}

fn on_simulation_unpaused(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut pause_event_writer: EventWriter<MenuAction<PauseMenuUi>>,
) {
    let window = q_windows.single_mut();
    grab_cursor(window);
    pause_event_writer.send(MenuAction::Hide);
}

fn release_cursor(mut window: Mut<Window>) {
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

fn grab_cursor(mut window: Mut<Window>) {
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}
