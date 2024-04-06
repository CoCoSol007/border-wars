//! The file that contains utility functions, enums, structs for the game.

use bevnet::Uuid;
use bevy::prelude::*;
use networking::PlayerRank;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod camera;
pub mod map;
pub mod networking;
pub mod scenes;
pub mod ui;

/// A scene of the game.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Scene {
    /// When we are in the main menu.
    #[default]
    Menu,

    /// When we are in the lobby waiting for players to join the game.
    Lobby,

    /// When we play this wonderful game.
    Game,
}

/// The current scene of the game.
pub type CurrentScene = Scene;

/// A player in the game.
#[derive(Serialize, Deserialize, Clone, Debug, Component, Resource, PartialEq, Eq, Hash)]
pub struct Player {
    /// The name of the player.
    pub name: String,

    /// The rank of the player.
    pub rank: PlayerRank,

    /// The uuid of the player.
    pub uuid: Uuid,

    /// The color of the player.
    pub color: (u8, u8, u8),
}
