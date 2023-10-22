use bevy::prelude::*;

use super::systems::main_menu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu);
    }
}
