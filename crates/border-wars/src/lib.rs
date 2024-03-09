//! The file that contains utility functions, enums, structs for the game.

use bevy::prelude::*;

pub mod camera;
pub mod map;
pub mod responsive_scale;
pub mod scenes;

/// The current scene of the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum CurrentScene {
    /// When we are in the main menu.
    #[default]
    Menu,

    /// When we are in the lobby waiting for players to join the game.
    Lobby,

    /// When we play this wonderful game.
    Game,

    /// When we are in the settings menu.
    Setting,
}

/// Calculates the ui_scale.0 depending on the size of the main node
pub fn change_scaling(mut ui_scale: ResMut<UiScale>, window: Query<&Window>) {
    let window = window.single();
    let (a, b) = (
        window.resolution.width() / 1280.,
        window.resolution.height() / 720.,
    );
    ui_scale.0 = if a < b { a } else { b } as f64
}
