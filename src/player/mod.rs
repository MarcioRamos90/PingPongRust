use bevy::{prelude::*, window::PrimaryWindow};

pub mod components;
pub mod systems;

use systems::*;

use crate::boll::components::{Acceleration, Boll};

pub struct PlayersPlugin;

const WIDTH_PLAYER: f32 = 30.;
const HEIGHT_PLAYER: f32 = 100.;

const VELOCITY_PLAYER: f32 = 10.;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(players_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(hit_boll_system);
    }
}

