use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{events::GameWin, player::components::Player};

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

pub fn handle_game_win(mut game_winner_event_reader: EventReader<GameWin>) {
    for event in game_winner_event_reader.iter() {
        println!("event: {:?}", event.player);

        let winner: &str = match event.player {
            Player::P1 => "Player 1",
            Player::P2 => "Player 2",
            _ => "No Winner",
        };
        println!("The winner are: {} !!!", winner);
    }
}
