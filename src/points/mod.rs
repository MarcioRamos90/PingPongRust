use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;


pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Points>()
            .add_startup_system(points_text_spawn_system)
            .add_system(update_points);
    }
}
