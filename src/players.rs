 use  bevy::{prelude::*, window::PrimaryWindow};

 use crate::{
    components::Player,
 };

 pub struct PlayersPlugin;

 impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(players_spawn_system)
        .add_system(player_keyboard_event_system);
    }
 }


pub fn players_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn_batch(vec![
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.25, 0.25),
                    custom_size: Some(Vec2::new(30., 100.)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0 + 30., window.height() / 2.0, 0.0),
                ..default()
            },
            Player::P1,
        ),
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(-1.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(-30.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_xyz(window.width() - 30., window.height() / 2.0, 0.0),
                ..default()
            },
            Player::P2,
        ),
    ]);
}

fn player_keyboard_event_system(mut query: Query<(&mut Player, &mut Transform)>, keys: Res<Input<KeyCode>>) {
    for (p, mut t) in &mut query.iter_mut() {
        match *p {
            Player::P1 => {
                if keys.pressed(KeyCode::W) {
                    t.translation.y += 12.;
                }

                if keys.pressed(KeyCode::S) {
                    t.translation.y -= 12.;
                }
            }
            Player::P2 => {
                if keys.pressed(KeyCode::Up) {
                    t.translation.y += 12.;
                }

                if keys.pressed(KeyCode::Down) {
                    t.translation.y -= 12.;
                }
            }
        }
    }
}