use bevy::prelude::Component;

#[derive(Component)]
pub enum Player {
    P1,
    P2,
}

#[derive(Component)]
pub struct Boll;

#[derive(Component)]
pub struct Acceleration{
    pub x: f32,
    pub y: f32,
}
