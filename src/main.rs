use std::env;
use bevy::prelude::*;

use game_of_life::GameOfLifeComputePlugin;

mod game_of_life;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            ..default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLifeComputePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}