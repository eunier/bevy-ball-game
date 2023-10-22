use super::constants::ENEMY_SPAWN_TIME;
use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
