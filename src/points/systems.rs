use bevy::prelude::*;

use super::resources::*;

pub fn update_points(points: Res<Points>) {
    if points.is_changed() {
        println!("Points: p1: {} ------ p2: {}",  points.p1, points.p2);
    }
}