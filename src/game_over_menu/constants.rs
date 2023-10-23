use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const GAME_OVER_MENU_STYLE: Style = Style {
    align_items: AlignItems::Center,
    display: Display::Flex, // Hidden by Default
    justify_content: JustifyContent::Center,
    position_type: PositionType::Absolute, // Needed to display separately from HUD.
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = Style {
    align_items: AlignItems::Center,
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    justify_content: JustifyContent::Center,
    size: Size::new(Val::Px(400.0), Val::Px(400.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};
