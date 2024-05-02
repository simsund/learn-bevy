use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod enemy;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((enemy::EnemyPlugin, player::PlayerPlugin))
        .add_systems(Startup, spawn_camera)
        .run()
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
