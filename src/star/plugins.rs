use bevy::prelude::{App, Plugin};

use super::{
    resources::StarSpawnTimer,
    systems::{spawn_star, spawn_stars_over_time, tick_star_spawn_time},
};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_star)
            .add_system(tick_star_spawn_time)
            .add_system(spawn_stars_over_time);
    }
}
