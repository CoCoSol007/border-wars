//! The main entry point of the game.

use bevy::prelude::*;
use border_wars::camera::CameraPlugin;
use border_wars::map::click_tile::TilesClickable;
use border_wars::map::renderer::RendererPlugin;
use border_wars::scenes::ScenesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .add_plugins(RendererPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TilesClickable)
        .run();
}
