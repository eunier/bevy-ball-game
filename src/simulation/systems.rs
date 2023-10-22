use bevy::prelude::*;

use super::states::SimulationState;

pub fn toggle_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }

        if simulation_state.0 == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}
