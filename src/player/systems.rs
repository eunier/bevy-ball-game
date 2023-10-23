use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    score::resources::Score,
    star::{components::Star, constants::START_SIZE},
};

use super::{
    components::Player,
    constants::{PLAYER_SIZE, PLAYER_SPEED},
};

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

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_hit_start(
    mut commands: Commands,
    mut score: ResMut<Score>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (start_entity, start_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(start_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = START_SIZE / 2.0;
            let did_player_hit_star = distance < player_radius + star_radius;

            if did_player_hit_star {
                info!("Player huit star!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/impactWood_medium_000.ogg");
                audio.play(sound_effect);
                commands.entity(start_entity).despawn();
            }
        }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

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
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if is_right_key_pressed {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if is_up_key_pressed {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if is_down_key_pressed {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

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

        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        } else if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }

        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        } else if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
    }
}
