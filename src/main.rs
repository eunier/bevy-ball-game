mod app;
mod camera;
mod enemy;
mod exit;
mod game_over;
mod game_over_menu;
mod high_score;
mod main_menu;
mod pause_menu;
mod player;
mod score;
mod simulation;
mod star;
mod hud;

use app::plugins::AppStatePlugin;
use bevy::{prelude::App, DefaultPlugins};
use camera::plugins::CameraPlugin;
use enemy::plugins::EnemyPlugin;
use exit::plugins::ExitPlugin;
use game_over::plugins::GameOverPlugin;
use game_over_menu::plugins::GameOverMenuPlugin;
use high_score::plugins::HighScorePlugin;
use hud::puglins::HudPlugin;
use main_menu::plugins::MainMenuPlugin;
use pause_menu::plugins::PauseMenuPlugin;
use player::plugins::PlayerPlugin;
use score::plugins::ScorePlugin;
use simulation::plugins::SimulationPlugin;
use star::plugins::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AppStatePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ExitPlugin)
        .add_plugin(GameOverMenuPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(HighScorePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(PauseMenuPlugin)
        .run();
}
