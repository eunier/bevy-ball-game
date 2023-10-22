use bevy::prelude::*;

use super::{
    states::AppState,
    systems::{transition_to_game_state, transition_to_main_menu_state},
};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_system(transition_to_game_state)
            .add_system(transition_to_main_menu_state);
    }
}
