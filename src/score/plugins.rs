use bevy::prelude::{App, Plugin};

use super::{resources::Score, systems::update_score};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_system(update_score);
    }
}
