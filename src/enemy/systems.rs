use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::{
    game_over::events::GameOver,
    player::{components::Player, constants::PLAYER_SIZE},
    score::resources::Score,
};

use super::{
    components::Enemy,
    constants::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES},
    resources::EnemySpawnTimer,
};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let sprite_bundle = SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        };

        let enemy = Enemy {
            direction: Vec2::new(
                random::<f32>() * (if random::<f32>() > 0.5 { 1.0 } else { -1.0 }),
                random::<f32>() * (if random::<f32>() > 0.5 { 1.0 } else { -1.0 }),
            )
            .normalize(),
        };

        commands.spawn((sprite_bundle, enemy));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = PLAYER_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut is_direction_changed = false;

        if transform.translation.x < x_min || transform.translation.x > x_max {
            enemy.direction.x *= -1.0;
            is_direction_changed = true;
        }

        if transform.translation.y < y_min || transform.translation.y > y_max {
            enemy.direction.y *= -1.0;
            is_direction_changed = true;
        }

        if is_direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            audio.play(sound_effect);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut enemy_transform in enemy_query.iter_mut() {
        if enemy_transform.translation.x < x_min {
            enemy_transform.translation.x = x_min;
        } else if enemy_transform.translation.x > x_max {
            enemy_transform.translation.x = x_max;
        }

        if enemy_transform.translation.y < y_min {
            enemy_transform.translation.y = y_min;
        } else if enemy_transform.translation.y > y_max {
            enemy_transform.translation.y = y_max;
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    enemy_query: Query<&Transform, With<Enemy>>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            let did_enemy_hit_player = distance < player_radius + enemy_radius;

            if did_enemy_hit_player {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn tick_enemy_spawn_time(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        let sprite_bundle = SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        };

        let enemy = Enemy {
            direction: Vec2::new(
                random::<f32>() * (if random::<f32>() > 0.5 { 1.0 } else { -1.0 }),
                random::<f32>() * (if random::<f32>() > 0.5 { 1.0 } else { -1.0 }),
            )
            .normalize(),
        };

        commands.spawn((sprite_bundle, enemy));
    }
}
