use bevy::prelude::*;

use crate::app::states::AppState;

use super::events::GameOver;

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        println!("Your final score is: {}", event.score.to_string());
    }
}
