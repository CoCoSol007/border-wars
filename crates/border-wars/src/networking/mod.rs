//! All the code related to the networking.

use bevnet::{NetworkAppExt, NetworkPlugin, Receive};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use self::check_connection::CheckConnectionPlugin;
use self::connection::ConnectionPlugin;
use crate::map::generation::StartMapGeneration;
use crate::CurrentScene;

pub mod check_connection;
pub mod connection;
pub mod reconnection;

/// The plugin for the networking.
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkPlugin::new("relay.cocosol.fr".to_string()))
            .add_plugins(ConnectionPlugin)
            .add_systems(Update, handle_start_game)
            .add_network_event::<StartGame>()
            .add_plugins(CheckConnectionPlugin);
    }
}

/// The rank of the player.
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug, Hash)]
pub enum PlayerRank {
    /// A spectator. He does not play the game, just renderer the game.
    Spectator,

    /// An admin. He manages the game and play the game.
    Admin,

    /// The player. He can join the game and play.
    Player,
}

/// The event to start the game, that is send by the admin.
#[derive(Event, Serialize, Deserialize)]
pub struct StartGame(pub StartMapGeneration);

/// A fonction that handle the start of the game.
fn handle_start_game(
    mut next_stats: ResMut<NextState<CurrentScene>>,
    mut start_game_events: EventReader<Receive<StartGame>>,
    mut start_map_generation_writer: EventWriter<StartMapGeneration>,
) {
    for event in start_game_events.read() {
        next_stats.set(CurrentScene::Game);
        start_map_generation_writer.send(event.1.0);
    }
}
