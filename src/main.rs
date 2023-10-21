use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows_query.get_single().unwrap();
}
