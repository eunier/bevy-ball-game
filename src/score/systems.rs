use super::resources::Score;
use bevy::prelude::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        info!("Score: {}", score.value.to_string());
    }
}

pub fn inser_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}
