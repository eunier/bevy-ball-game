mod camera;
mod enemy;
mod exit;
mod game_over;
mod high_score;
mod player;
mod score;
mod star;

use bevy::{prelude::App, DefaultPlugins};
use camera::plugins::CameraPlugin;
use enemy::plugins::EnemyPlugin;
use exit::plugins::ExitPlugin;
use game_over::plugins::GameOverPlugin;
use high_score::plugins::HighScorePlugin;
use player::plugins::PlayerPlugin;
use score::plugins::ScorePlugin;
use star::plugins::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ExitPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(HighScorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .run();
}
