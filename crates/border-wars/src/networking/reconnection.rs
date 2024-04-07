//! All program related to reconnection logic.

use bevnet::NetworkAppExt;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::ownership::Owner;
use crate::map::{Tile, TilePosition};
use crate::Player;

/// The plugin for the reconnection.
pub struct ReconnectionPlugin;

impl Plugin for ReconnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_network_event::<ReconnectionRequest>()
            .add_network_event::<ReconnectionResponse>();
    }
}

/// The event to request the reconnection.
#[derive(Event, Serialize, Deserialize)]
pub struct ReconnectionRequest;

/// The event to response to the reconnection.
#[derive(Event, Serialize, Deserialize)]
pub struct ReconnectionResponse {
    /// All the players in the game.
    pub players: Vec<Player>,

    /// The map of the game.
    pub map: Vec<(Option<Owner>, Tile, TilePosition)>,

    /// The current player.
    pub current_player: Player,
}
