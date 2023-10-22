use bevy::prelude::Plugin;

use super::{events::GameOver, systems::handle_game_over};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameOver>().add_system(handle_game_over);
    }
}
