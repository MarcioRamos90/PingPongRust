use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
enum Player {
    P1,
    P2,
}

#[derive(Component)]
struct Boll;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(boll_mov)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Boll,
    ));

    commands.spawn_batch(vec![
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(30.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-200., 0., 0.),
                ..default()
            },
            Player::P1,
        ),
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(-30.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_xyz(200., 0., 0.),
                ..default()
            },
            Player::P2,
        ),
    ])
}

fn keyboard_input(mut query: Query<(&mut Player, &mut Transform)>, keys: Res<Input<KeyCode>>) {
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


fn boll_mov(mut query: Query<(&mut Boll,&mut Transform)>) {
    for (_,  mut t) in query.iter_mut() {
        t.translation.x += 1.;
        t.translation.y += 1.;
    }
}