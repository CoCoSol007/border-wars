//! All the code related to the networking.

use bevnet::NetworkPlugin;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use self::connection::ConnectionPlugin;

pub mod connection;

/// The plugin for the networking.
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkPlugin::new("relay.cocosol.fr".to_string()))
            .add_plugins(ConnectionPlugin);
    }
}

/// The rank of the player.
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PlayerRank {
    /// A spectator. He does not play the game, just renderer the game.
    Spectator,

    /// An admin. He manages the game and play the game.
    Admin,

    /// The player. He can join the game and play.
    Player,
}
