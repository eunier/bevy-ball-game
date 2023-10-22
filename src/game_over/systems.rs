use bevy::prelude::*;

use crate::app::states::AppState;

use super::events::GameOver;

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        next_app_state.set(AppState::GameOver);
        println!("Your final score is: {}", event.score.to_string());
    }
}
