//! The main entry point of the game.

use bevy::prelude::*;
use border_wars::menu::MenuPlugin;
use border_wars::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugins(MenuPlugin)
        .run();
}
