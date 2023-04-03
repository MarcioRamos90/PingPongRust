#![allow(unused)]

use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};

pub mod player;
pub mod boll;
pub mod points;

use player::PlayersPlugin;
use boll::BollPlugin;
use points::PointsPlugin;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayersPlugin)
    .add_plugin(BollPlugin)
    .add_plugin(PointsPlugin)
    .add_startup_system(spawn_camera)
    .add_system(exit_game)
    .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}