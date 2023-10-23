use bevy::prelude::*;

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        color: Color::rgb(1.0, 1.0, 1.0),
        font_size: 64.0,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        color: Color::rgb(1.0, 1.0, 1.0),
        font_size: 48.0,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        color: Color::rgb(1.0, 1.0, 1.0),
        font_size: 32.0,
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    }
}
