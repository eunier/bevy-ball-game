use bevy::prelude::*;

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn(NodeBundle {
            background_color: Color::RED.into(),
            ..default()
        })
        .id();

    main_menu_entity
}
