//! The main entry point of the game.

use bevy::prelude::*;
use border_wars::camera::CameraPlugin;
use border_wars::map::click_tile::TilesClickable;
use border_wars::map::generation::{MapGenerationPlugin, StartMapGeneration};
use border_wars::map::renderer::RendererPlugin;
use border_wars::map::selected_tile::SelectTilePlugin;
use border_wars::scenes::ScenesPlugin;
use border_wars::ui::tiles_info::TilesInfoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .add_plugins(RendererPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(TilesClickable)
        .add_plugins(MapGenerationPlugin)
        .add_systems(OnEnter(border_wars::CurrentScene::Game), start)
        .add_plugins(TilesInfoPlugin)
        .run();
}

fn start(mut event: EventWriter<StartMapGeneration>) {
    event.send(StartMapGeneration {
        seed: 0,
        radius: 10,
    });
}
