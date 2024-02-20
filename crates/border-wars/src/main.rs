//! The main entry point of the game.

use bevy::{prelude::*, text::TextSettings};
use border_wars::scenes::ScenesPlugin;

fn main() {
    App::new()
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .run();
}
