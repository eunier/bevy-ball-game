use bevy::prelude::*;

use super::states::AppState;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}
