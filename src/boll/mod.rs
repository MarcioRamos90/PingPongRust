use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub mod systems;
pub mod components;

use systems::*;

const BOLL_SIZE: f32 = 30.;

pub struct BollPlugin;

impl Plugin for BollPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(boll_spawn_system)
            .add_system(boll_mov_system)
            .add_system(hit_corners)
            .add_system(point_count_system);
    }
}

