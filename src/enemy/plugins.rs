use bevy::prelude::Plugin;

use super::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, enemy_hit_player, enemy_movement, spawn_enemies,
        spawn_enemies_over_time, tick_enemy_spawn_time, update_enemy_direction,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(spawn_enemies)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(enemy_hit_player)
            .add_system(tick_enemy_spawn_time)
            .add_system(spawn_enemies_over_time);
    }
}
