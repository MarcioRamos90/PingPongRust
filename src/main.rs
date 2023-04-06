#![allow(unused)]

use bevy::prelude::*;

pub mod events;
pub mod rules;
mod systems;

mod boll;
mod player;
mod points;

use events::*;
use systems::*;

use boll::BollPlugin;
use player::PlayersPlugin;
use points::PointsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_event::<GameWin>()
        .add_plugin(PlayersPlugin)
        .add_plugin(BollPlugin)
        .add_plugin(PointsPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_win)
        .run();
}
