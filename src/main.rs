use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

mod components;
mod players;

use players::PlayersPlugin;

#[derive(Component)]
struct Boll;

#[derive(Component)]
struct Acceleration{
    x: f32,
    y: f32,
}


fn main() {
    App::new()
    .add_startup_system(setup_system)
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayersPlugin)
    .add_system(boll_mov)
    .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    
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

fn boll_mov(mut query: Query<(&mut Boll,&mut Transform, &mut Acceleration)>) {
    for (_,  mut t, a) in query.iter_mut() {
        t.translation.x += a.x;
        t.translation.y += a.y;
    }
}