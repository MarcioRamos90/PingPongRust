use bevy::{prelude::*, window::PrimaryWindow};

use super::resources::*;

#[derive(Debug, Component)]
pub enum PointsType {
    P1,
    P2,
}

#[derive(Component)]
pub struct P2Text;

pub fn points_text_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    points: Res<Points>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        TextBundle::from_section(
            format!("Player 1: {}", points.p1),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::YELLOW_GREEN,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        PointsType::P1,
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("Player 2: {}", points.p2),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::YELLOW_GREEN,
            },
        ) 
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        PointsType::P2,
    ));
}

pub fn update_points(
    points: Res<Points>,
    mut mut_query_p1_txt: Query<(&mut Text, &PointsType)>,
    asset_server: Res<AssetServer>,
) {
    if points.is_changed() {
        for (mut t, p ) in &mut mut_query_p1_txt {
            match *p {
                PointsType::P1 =>  {
                   t.sections[0].value = format!("Player 1: {}", points.p1);
               }
               PointsType::P2 => {
                   t.sections[0].value = format!("Player 2: {}", points.p2);
               } 
            }
        }
        println!("Points: p1: {} ------ p2: {}", points.p1, points.p2);
    }
}
