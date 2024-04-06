//! All the code related to the connection.

use bevnet::{Connection, NetworkAppExt, Receive, SendTo};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::PlayerRank;
use crate::{CurrentScene, Player};

/// A plugin that manage connections (add, remove).
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_network_event::<RequestJoin>()
            .add_network_event::<AddPlayer>()
            .add_network_event::<RemovePlayer>()
            .add_systems(
                Update,
                (accept_connection, handle_new_player, handle_remove_player),
            );
    }
}

/// An event that is trigger when a new player request to join a game.
#[derive(Event, Serialize, Deserialize)]
pub struct RequestJoin(pub Player);

/// An event that is trigger when a new player is added.
#[derive(Event, Serialize, Deserialize)]
pub struct AddPlayer(Player);

/// An event that is trigger when a player is removed.
#[derive(Event, Serialize, Deserialize)]
pub struct RemovePlayer(pub Player);

/// A fonction that accept new connection.
/// It add the player to the list of all players.
pub fn accept_connection(
    all_players_query: Query<&Player>,
    mut requests_join_event: EventReader<Receive<RequestJoin>>,
    mut add_players_event: EventWriter<SendTo<AddPlayer>>,
    state: Res<State<CurrentScene>>,
) {
    for request_join in requests_join_event.read() {
        let mut new_player = request_join.1.0.clone();

        let current_state = *state.get();

        if current_state == CurrentScene::Menu {
            return;
        } else if current_state == CurrentScene::Game {
            new_player.rank = PlayerRank::Spectator;
        }

        add_players_event.send(SendTo(new_player.uuid, AddPlayer(new_player.clone())));

        for old_player in all_players_query.iter() {
            // Link all players
            add_players_event.send(SendTo(old_player.uuid, AddPlayer(new_player.clone())));
            add_players_event.send(SendTo(new_player.uuid, AddPlayer(old_player.clone())));
        }
    }
}

/// A fonction that handle new players when a events is received.
pub fn handle_new_player(mut add_players: EventReader<Receive<AddPlayer>>, mut commands: Commands) {
    for add_player in add_players.read() {
        commands.spawn(add_player.1.0.clone());
    }
}

/// A fonction that handle remove players when a events is received.
pub fn handle_remove_player(
    mut remove_players: EventReader<Receive<RemovePlayer>>,
    mut commands: Commands,
    all_players_query: Query<(Entity, &Player)>,
    connection: Res<Connection>,
    mut next_scene: ResMut<NextState<CurrentScene>>,
) {
    for remove_player in remove_players.read() {
        if Some(remove_player.1.0.uuid) == connection.identifier() {
            next_scene.set(CurrentScene::Menu);
            all_players_query.iter().for_each(|(entity, _)| {
                commands.entity(entity).despawn();
            });
            return;
        }
        for (entity, player) in all_players_query.iter() {
            if remove_player.1.0.uuid == player.uuid {
                commands.entity(entity).despawn();
            }
        }
    }
}
