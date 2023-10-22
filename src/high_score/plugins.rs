use bevy::prelude::Plugin;

use super::{
    resources::HighScores,
    systems::{high_scores_updated, update_high_scores},
};

pub struct HighScorePlugin;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<HighScores>()
            .add_system(update_high_scores)
            .add_system(high_scores_updated);
    }
}
