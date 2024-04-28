use std::collections::hash_map::{Entry, VacantEntry};
use std::collections::HashMap;

use lazy_static::lazy_static;
use server::{Lobby, Player, ServerMessage};
use tokio::sync::mpsc::Sender;
use tokio::sync::{RwLock, RwLockWriteGuard};
use uuid::Uuid;

use crate::{ClientEvent, ClientMessage, CONNECTIONS};

#[derive(Clone, Copy)]
pub enum ClientStatus {
    InLobby(Uuid),
    InGame(Uuid),
}

lazy_static! {
    pub static ref CLIENT_STATUS: RwLock<HashMap<Uuid, ClientStatus>> = RwLock::new(HashMap::new());
    pub static ref LOBBIES: RwLock<HashMap<Uuid, Lobby>> = RwLock::new(HashMap::new());
}

pub async fn handle_event(id: Uuid, event: ClientEvent) {
    let status = CLIENT_STATUS.read().await.get(&id).copied();
    match status {
        None => handle_unauthenticated_event(id, event).await,
        Some(ClientStatus::InLobby(lobby_id)) => handle_lobby_event(lobby_id, id, event).await,
        Some(ClientStatus::InGame(game_id)) => handle_game_event(game_id, id, event).await,
    }
}

async fn handle_unauthenticated_event(id: Uuid, event: ClientEvent) {
    match event {
        ClientEvent::Message(ClientMessage::CreateLobby { username, public }) => {
            // Create the lobby
            let mut lobbies = LOBBIES.write().await;
            let lobby_id = Uuid::now_v7();
            let lobby = lobbies.entry(lobby_id).or_insert(Lobby {
                id: lobby_id,
                public,
                players: HashMap::from_iter([(
                    id,
                    Player {
                        id,
                        username,
                        ready: false,
                    },
                )]),
            });

            // Change the client status
            CLIENT_STATUS
                .write()
                .await
                .insert(id, ClientStatus::InLobby(lobby.id));

            // Send the lobby update
            CONNECTIONS
                .send(id, ServerMessage::LobbyUpdate(lobby.clone()))
                .await;
        }
        ClientEvent::Message(ClientMessage::JoinLobby { username, lobby_id }) => {
            // Find or create the lobby
            let mut lobbies = LOBBIES.write().await;
            let lobby = match lobby_id {
                Some(lobby_id) => {
                    let Some(lobby) = lobbies.get_mut(&lobby_id) else {
                        CONNECTIONS
                            .send(id, ServerMessage::Refused("Lobby not found".to_string()))
                            .await;
                        return;
                    };
                    lobby
                }
                None => {
                    if let Some((_, lobby)) = lobbies
                        .iter_mut()
                        .min_by_key(|(_, value)| value.players.len())
                    {
                        lobby
                    } else {
                        let lobby_id = Uuid::now_v7();
                        lobbies.entry(lobby_id).or_insert(Lobby {
                            id: lobby_id,
                            public: true,
                            players: HashMap::new(),
                        })
                    }
                }
            };

            // Change the client status
            CLIENT_STATUS
                .write()
                .await
                .insert(id, ClientStatus::InLobby(lobby.id));

            // Add the player to the lobby
            lobby.players.insert(
                id,
                Player {
                    id,
                    username,
                    ready: false,
                },
            );

            // Send the lobby update to all players
            let message = ServerMessage::LobbyUpdate(lobby.clone());
            for player_id in lobby.players.keys() {
                CONNECTIONS.send(*player_id, &message).await;
            }
        }
        _ => (),
    };
}

async fn handle_lobby_event(lobby_id: Uuid, id: Uuid, event: ClientEvent) {
    match event {
        ClientEvent::Message(ClientMessage::Ready(ready)) => {
            // Get the lobby
            let mut lobbies = LOBBIES.write().await;
            let Some(lobby) = lobbies.get_mut(&lobby_id) else {
                return;
            };

            // Get the lobby player
            let Some(player) = lobby.players.get_mut(&id) else {
                return;
            };

            // Update the player ready status
            player.ready = ready;

            // If everyone is ready, start the game
            if lobby.players.len() >= 2 && lobby.players.values().all(|p| p.ready) {
                todo!("start the game");
                return;
            }

            // Send the lobby update to all players
            let message = ServerMessage::LobbyUpdate(lobby.clone());
            for player_id in lobby.players.keys() {
                CONNECTIONS.send(*player_id, &message).await;
            }
        }
        ClientEvent::Disconnected => {
            // Remove the client status
            CLIENT_STATUS.write().await.remove(&id);

            // Remove the client from the lobby
            let mut lobbies = LOBBIES.write().await;
            let Some(lobby) = lobbies.get_mut(&lobby_id) else {
                return;
            };
            lobby.players.remove(&id);

            // If the lobby is empty, remove it
            if lobby.players.is_empty() {
                lobbies.remove(&lobby_id);
                return;
            }

            // If everyone is ready, start the game
            if lobby.players.len() >= 2 && lobby.players.values().all(|p| p.ready) {
                todo!("start the game");
                return;
            }

            // Send the lobby update to all players
            let message = ServerMessage::LobbyUpdate(lobby.clone());
            for player_id in lobby.players.keys() {
                CONNECTIONS.send(*player_id, &message).await;
            }
        }
        _ => (),
    }
}

async fn handle_game_event(game_id: Uuid, id: Uuid, event: ClientEvent) {
    todo!()
}
