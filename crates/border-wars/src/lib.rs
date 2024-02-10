//! The file that contains utility functions, enums, structs for the game.

use bevy::prelude::*;

pub mod scenes;

/// The current scene of the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CurrentScene {
    /// When we are in the main menu.
    #[default]
    Menu,

    /// When we are in the lobby waiting for players to join the game.
    Lobby,

    /// When we play this wonderful game.
    Game,
}
