use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

pub const HUD_STYLE: Style = Style {
    align_items: AlignItems::Center,
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
    ..Style::DEFAULT
};

pub const LHS_STYLE: Style = Style {
    align_items: AlignItems::Center,
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    ..Style::DEFAULT
};

pub const RHS_STYLE: Style = Style {
    align_items: AlignItems::Center,
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    size: Size::new(Val::Px(48.0), Val::Px(48.0)),
    ..Style::DEFAULT
};
