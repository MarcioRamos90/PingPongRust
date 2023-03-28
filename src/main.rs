use bevy::prelude::*;

mod components;
mod players;
mod boll;

use players::PlayersPlugin;
use boll::BollPlugin;


fn main() {
    App::new()
    .add_startup_system(setup_system)
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayersPlugin)
    .add_plugin(BollPlugin)
    .run();
}

fn setup_system(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}