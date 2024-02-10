//! The file containing all scenes programs.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::CurrentScene;

pub mod menu;

/// The plugin for all scenes.
pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_state::<CurrentScene>()
            .add_plugins(menu::MenuPlugin);
    }
}
