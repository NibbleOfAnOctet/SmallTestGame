use bevy::prelude::*;
use levels::level1;
use player::Player;
mod player;
mod levels;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(input::InputBindings::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            Player::systems(),
            levels::debug_switch_level,
        ).chain())
        .run();
}

fn setup(commands: Commands) {
    level1::load_level(commands)
}

