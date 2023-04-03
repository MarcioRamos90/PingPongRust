use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Player};
use super::{WIDTH_PLAYER, HEIGHT_PLAYER, VELOCITY_PLAYER};

use crate::boll::components::{Acceleration, Boll};

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
                    custom_size: Some(Vec2::new(WIDTH_PLAYER, HEIGHT_PLAYER)),
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
                    custom_size: Some(Vec2::new(WIDTH_PLAYER, HEIGHT_PLAYER)),
                    ..default()
                },
                transform: Transform::from_xyz(window.width() - 30., window.height() / 2.0, 0.0),
                ..default()
            },
            Player::P2,
        ),
    ]);
}

pub fn player_keyboard_event_system(
    mut query: Query<(&mut Player, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    
    for (p, mut t) in &mut query.iter_mut() {
        
        let hit_up_wall: bool = win_h <= t.translation.y + HEIGHT_PLAYER/2.;
        let hit_down_wall: bool = 0. >= t.translation.y - HEIGHT_PLAYER/2.;
        
        match *p {
            Player::P1 => {
                if keys.pressed(KeyCode::W) && !hit_up_wall{
                    t.translation.y += VELOCITY_PLAYER;
                }

                if keys.pressed(KeyCode::S) && !hit_down_wall {
                    t.translation.y -= VELOCITY_PLAYER;
                }
            }
            Player::P2 => {
                if keys.pressed(KeyCode::Up) && !hit_up_wall {
                    t.translation.y += VELOCITY_PLAYER;
                }

                if keys.pressed(KeyCode::Down) && !hit_down_wall {
                    t.translation.y -= VELOCITY_PLAYER;
                }
            }
        }
    }
}

pub fn hit_boll_system(
    query_players: Query<(&Transform, &Player)>,
    mut query_boll: Query<(
        &mut Transform,
        &mut Acceleration,
        With<Boll>,
        Without<Player>,
    )>,
) {
    let (mut boll_translation, mut boll_acceleration, _, _) = query_boll.get_single_mut().unwrap();



    for (t, player) in query_players.iter() {
        match player {
            Player::P1 => {
                if (boll_translation.translation.x <= t.translation.x + WIDTH_PLAYER
                    && boll_translation.translation.x >= t.translation.x)
                    && (t.translation.y - HEIGHT_PLAYER <= boll_translation.translation.y
                        && t.translation.y + HEIGHT_PLAYER >= boll_translation.translation.y)
                {
                    boll_acceleration.x *= -1.;
                }
            }
            Player::P2 => {
                if (boll_translation.translation.x >= t.translation.x - WIDTH_PLAYER
                    && boll_translation.translation.x <= t.translation.x)
                    && (t.translation.y - HEIGHT_PLAYER <= boll_translation.translation.y
                        && t.translation.y + HEIGHT_PLAYER >= boll_translation.translation.y)
                {
                    boll_acceleration.x *= -1.;
                }
            }
        }
    }
}
