use bevy::{app::AppExit, prelude::*};

pub fn exit_game(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
