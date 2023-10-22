use bevy::prelude::{App, IntoSystemConfig, IntoSystemSetConfig, Plugin};

use super::systems::{
    confine_player_movement, player_hit_start, player_movement, spawn_player, ConfinementSystemSet,
    MovementSystemSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(player_hit_start)
            .add_system(player_movement.in_set(MovementSystemSet));
    }
}
