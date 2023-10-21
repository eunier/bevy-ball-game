use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let is_left_key_pressed =
            keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);

        let is_right_key_pressed =
            keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

        let is_up_key_pressed =
            keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);

        let is_down_key_pressed =
            keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);

        if is_left_key_pressed {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        if is_right_key_pressed {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if is_up_key_pressed {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        if is_down_key_pressed {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        match (
            player_transform.translation.x < x_min,
            player_transform.translation.x > x_max,
        ) {
            (true, _) => player_transform.translation.x = x_min,
            (_, true) => player_transform.translation.x = x_max,
            _ => (),
        }

        match (
            player_transform.translation.y < y_min,
            player_transform.translation.y > y_max,
        ) {
            (true, _) => player_transform.translation.y = y_min,
            (_, true) => player_transform.translation.y = y_max,
            _ => (),
        }
    }
}
