use bevy::prelude::*;

use crate::{app::states::AppState, simulation::states::SimulationState};

use super::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, despawn_enemies, enemy_hit_player, enemy_movement, spawn_enemies,
        spawn_enemies_over_time, tick_enemy_spawn_time, update_enemy_direction,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    enemy_hit_player,
                    tick_enemy_spawn_time,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
