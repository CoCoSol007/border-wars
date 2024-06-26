//! The main entry point of the game.

use bevy::prelude::*;
use border_wars::camera::CameraPlugin;
use border_wars::map::generation::MapGenerationPlugin;
use border_wars::map::ownership::OwnershipPlugin;
use border_wars::map::renderer::RendererPlugin;
use border_wars::map::selected_tile::SelectTilePlugin;
use border_wars::networking::NetworkingPlugin;
use border_wars::resources::ResourcesPlugin;
use border_wars::scenes::ScenesPlugin;
use border_wars::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .add_plugins(RendererPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SelectTilePlugin)
        .add_plugins(NetworkingPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(OwnershipPlugin)
        .add_plugins(ResourcesPlugin)
        .run();
}
