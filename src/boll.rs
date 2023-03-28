use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{components::{Boll, Acceleration}};


pub struct BollPlugin;

impl Plugin for BollPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(boll_spawn_system)
        .add_system(boll_mov_system);
    }
}

pub fn boll_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let x: bool = rng.gen();
    let y: bool = rng.gen();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Boll,
        Acceleration {
            x: if x {1.} else {-1.},
            y: if y {1.} else {-1.},
        },
    ));
}

pub fn boll_mov_system(mut query: Query<(&mut Boll,&mut Transform, &mut Acceleration)>) {
    for (_,  mut t, a) in query.iter_mut() {
        t.translation.x += a.x;
        t.translation.y += a.y;
    }
}