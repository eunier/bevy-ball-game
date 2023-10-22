use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::{app::states::AppState, simulation::states::SimulationState};

use super::{
    resources::StarSpawnTimer,
    systems::{despawn_star, spawn_star, spawn_stars_over_time, tick_star_spawn_time},
};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_star.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (spawn_stars_over_time, tick_star_spawn_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_star.in_schedule(OnExit(AppState::Game)));
    }
}
