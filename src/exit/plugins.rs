use bevy::prelude::Plugin;

use super::systems::exit_game;

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(exit_game);
    }
}
