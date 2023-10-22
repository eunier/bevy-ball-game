use bevy::prelude::*;

use crate::{app::states::AppState, simulation::states::SimulationState};

use super::systems::{
    confine_player_movement, despawn_player, player_hit_start, player_movement, spawn_player,
    ConfinementSystemSet, MovementSystemSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    confine_player_movement.in_set(ConfinementSystemSet),
                    player_hit_start,
                    player_movement.in_set(MovementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
