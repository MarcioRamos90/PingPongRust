use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::Rng;

use crate::components::{Acceleration, Boll};

const BOLL_SIZE: f32 = 30.;

pub struct BollPlugin;

impl Plugin for BollPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(boll_spawn_system)
            .add_system(boll_mov_system)
            .add_system(hit_corners);
    }
}

pub fn boll_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::thread_rng();

    let x: bool = rng.gen();
    let y: bool = rng.gen();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BOLL_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Boll,
        Acceleration {
            x: if x { 5. } else { -5. },
            y: if y { 5. } else { -5. },
        },
    ));
}

pub fn boll_mov_system(mut query: Query<(&mut Boll, &mut Transform, &mut Acceleration)>) {
    for (_, mut t, a) in query.iter_mut() {
        t.translation.x += a.x;
        t.translation.y += a.y;
    }
}

pub fn hit_corners(
    mut boll_query: Query<(&mut Transform, &mut Acceleration, With<Boll>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let y_min: f32 = 0.0 + BOLL_SIZE as f32;
    let y_max: f32 = win_h - BOLL_SIZE as f32;

    for (mut t, mut acceleration, _) in boll_query.iter_mut() {
        if t.translation.y <= y_min {
            acceleration.y *= -1.;
        } else if t.translation.y >= y_max {
            acceleration.y *= -1.;
        }

        println!("w {} h {} t {:?}",  win_w, win_h, t.translation);
    }
}
