use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, Plugin,
};

use crate::app::states::AppState;

use super::systems::{inser_score, remove_score, update_score};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(inser_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
