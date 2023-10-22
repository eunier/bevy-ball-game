use bevy::prelude::*;

use crate::app::states::AppState;

use super::{
    states::SimulationState,
    systems::{pause_simulation, resume_simulation, toggle_simulation},
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}
