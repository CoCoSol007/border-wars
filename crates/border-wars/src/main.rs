//! The main entry point of the game.

use bevy::prelude::*;
use border_wars::scenes::ScenesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .run();
}
