use bevy::prelude::*;

use super::{states::SimulationState, systems::toggle_simulation};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_system(toggle_simulation);
    }
}
