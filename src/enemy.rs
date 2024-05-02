use bevy::{prelude::*, window::PrimaryWindow};
use bevy_audio::Volume;

use crate::player::{Player, PLAYER_SIZE};

pub const NUMBER_OF_ENEMIES: usize = 10;
pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_event::<EnemySoundQueue>()
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    enemy_sound_queue,
                    enemy_hit_player,
                ),
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Event)]
pub struct EnemySoundQueue(Entity);

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random(), rand::random()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy, Entity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_ev_writer: EventWriter<EnemySoundQueue>,
) {
    if let Ok(window) = window_query.get_single() {
        let half_enemy_size = ENEMY_SIZE / 2.0;
        let x_min = 0.0 + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0.0 + half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        for (transform, mut enemy, entity) in enemy_query.iter_mut() {
            let mut direction_changed = false;

            let translation = transform.translation;
            if translation.x < x_min || translation.x > x_max {
                enemy.direction.x *= -1.0;
                direction_changed = true;
            }
            if translation.y < y_min || translation.y > y_max {
                enemy.direction.y *= -1.0;
                direction_changed = true;
            }

            if direction_changed {
                enemy_ev_writer.send(EnemySoundQueue(entity));
            }
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut enemy_transform) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;
        let min = Vec2::splat(half_enemy_size).extend(f32::MIN);
        let max = Vec3::new(
            window.width() - half_enemy_size,
            window.height() - half_enemy_size,
            f32::MAX,
        );

        enemy_transform.translation = enemy_transform.translation.clamp(min, max);
    }
}

pub fn enemy_sound_queue(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemy_ev_reader: EventReader<EnemySoundQueue>,
) {
    for event in enemy_ev_reader.read() {
        commands.entity(event.0).insert(AudioBundle {
            source: asset_server.load("audio/pluck_001.ogg"),
            settings: PlaybackSettings::REMOVE.with_volume(Volume::new(0.1)),
        });
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    mut hitsound: EventWriter<EnemySoundQueue>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (enemy_transform, enemy_entity) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                hitsound.send(EnemySoundQueue(enemy_entity));
            }
        }
    }
}
