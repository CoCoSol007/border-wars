//! TODO

use std::time::Instant;

use bevnet::{Connection, NetworkAppExt, Receive, SendTo};
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::Player;

/// A plugin that check if a player is still connected.
pub struct CheckConnectionPlugin;

/// An event that is trigger when a player is disconnected.
#[derive(Event)]
pub struct PlayerDisconnected(pub Player);

/// An event that is send between all players to check if a player is still
/// connected.
#[derive(Event, Serialize, Deserialize)]
struct IAmConnected(Player);

impl Plugin for CheckConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_connection,
                send_check_connection,
                handle_disconnect_player,
            ),
        )
        .add_event::<PlayerDisconnected>()
        .add_network_event::<IAmConnected>();
    }
}

/// The interval to check if a player is still connected.
const CHECK_CONNECTION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

/// A fonction that check if a player is still connected.
fn check_connection(
    all_players_query: Query<&Player>,
    mut disconnect_event: EventWriter<PlayerDisconnected>,
    mut checked_players: Local<HashMap<Player, Instant>>,
    mut connect_event: EventReader<Receive<IAmConnected>>,
) {
    for Receive(_, IAmConnected(player)) in connect_event.read() {
        checked_players.insert(player.clone(), Instant::now());
    }
    for player in all_players_query.iter() {
        if !(*checked_players).contains_key(player) {
            checked_players.insert(player.clone(), Instant::now());
        }

        let Some(last_seen) = (*checked_players).get_mut(player) else {
            continue;
        };
        if last_seen.elapsed() > CHECK_CONNECTION_INTERVAL {
            disconnect_event.send(PlayerDisconnected(player.clone()));
            checked_players.remove(player);
        }
    }
}

/// A simple timer.
struct Timer(std::time::Duration, std::time::Instant);

impl Timer {
    /// Create a new timer.
    fn new(duration: std::time::Duration) -> Self {
        Self(duration, std::time::Instant::now())
    }

    /// Check if the timer is finished.
    fn is_finished(&self) -> bool {
        self.1.elapsed() >= self.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(CHECK_CONNECTION_INTERVAL / 2)
    }
}

/// A fonction that send a check connection event to all players.
fn send_check_connection(
    mut check_connection_event: EventWriter<SendTo<IAmConnected>>,
    all_players_query: Query<&Player>,
    connection: Res<Connection>,
    mut timer: Local<Timer>,
) {
    if !timer.is_finished() {
        return;
    }
    let Some(self_player) = all_players_query
        .iter()
        .find(|player| connection.identifier() == Some(player.uuid))
    else {
        return;
    };

    for player in all_players_query.iter() {
        check_connection_event.send(SendTo(player.uuid, IAmConnected(self_player.clone())));
    }

    timer.1 = std::time::Instant::now();
}

/// A fonction that handle player disconnection.
fn handle_disconnect_player(
    mut disconnect_players: EventReader<PlayerDisconnected>,
    all_players_query: Query<(&Player, Entity)>,
    mut commands: Commands,
) {
    for PlayerDisconnected(disconnect_player) in disconnect_players.read() {
        let Some((_, entity)) = all_players_query
            .iter()
            .find(|(player, _entity)| *player == disconnect_player)
        else {
            continue;
        };

        commands.entity(entity).despawn();
    }
}
