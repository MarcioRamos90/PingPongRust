
use bevy::prelude::*;

#[derive(Resource)]
pub struct Points {
    pub p1: u32,
    pub p2: u32,
}

impl Default for Points {
    fn default() -> Points {
        Points { p1: 0, p2: 0 }
    }
}
