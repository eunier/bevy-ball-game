use bevy::prelude::*;

use crate::app::states::AppState;

use super::systems::{
    despawn_main_menu, interact_with_play_button, interact_with_quit_button, spawn_main_menu,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
