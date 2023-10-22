use bevy::prelude::{App, Plugin};

use super::systems::{confine_player_movement, player_hit_start, player_movement, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_hit_start);
    }
}
