use std::borrow::{Borrow, Cow};
use std::collections::HashMap;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use log::warn;
use server::{ClientPacket, Lobby, LobbyPlayer, ServerPacket};
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::RwLock;
use uuid::Uuid;

struct Client {
    status: ClientStatus,
    sender: Sender<Vec<u8>>,
}

enum ClientStatus {
    Unauthenticated,
    InLobby(Uuid),
    InGame(Uuid),
}

lazy_static! {
    static ref CLIENTS: RwLock<HashMap<Uuid, Client>> = RwLock::new(HashMap::new());
    static ref LOBBIES: RwLock<HashMap<Uuid, Lobby>> = RwLock::new(HashMap::new());
}

pub async fn send_message<'a>(client_id: Uuid, message: impl Into<Cow<'a, [u8]>>) {
    if let Some(client) = CLIENTS.read().await.get(&client_id) {
        client.sender.send(message.into().into_owned()).await.ok();
    }
}

pub async fn send_packet(client_id: Uuid, packet: impl Borrow<ServerPacket>) {
    let message = match bincode::serialize(packet.borrow()) {
        Ok(message) => message,
        Err(error) => {
            warn!("failed to serialize packet for {}: {}", client_id, error);
            return;
        }
    };
    send_message(client_id, message).await;
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async { ws.on_upgrade(handle_client) }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

async fn handle_client(socket: WebSocket) {
    let client_id = Uuid::now_v7();

    let (mut sender, mut receiver) = socket.split();
    let (send_tx, mut send_rx) = channel(16);
    tokio::spawn(async move {
        while let Some(message) = send_rx.recv().await {
            sender.send(Message::Binary(message)).await?;
        }
        Ok::<(), axum::Error>(())
    });

    CLIENTS.write().await.insert(
        client_id,
        Client {
            status: ClientStatus::Unauthenticated,
            sender: send_tx,
        },
    );

    while let Some(Ok(message)) = receiver.next().await {
        let Message::Binary(message) = message else {
            continue;
        };
        let Ok(packet) = bincode::deserialize::<ClientPacket>(&message) else {
            warn!("failed to deserialize packet from {}", client_id);
            continue;
        };
        packet_received(client_id, packet).await;
    }

    packet_received(client_id, ClientPacket::Disconnect).await;
    CLIENTS.write().await.remove(&client_id);
}

async fn packet_received(client_id: Uuid, packet: ClientPacket) {
    let client = &CLIENTS.read().await[&client_id];
    match client.status {
        ClientStatus::Unauthenticated => handle_unauthenticated(client_id, packet).await,
        ClientStatus::InLobby(lobby_id) => handle_in_lobby(client_id, lobby_id, packet).await,
        ClientStatus::InGame(game_id) => handle_in_game(client_id, game_id, packet).await,
    }
}

async fn handle_unauthenticated(client_id: Uuid, packet: ClientPacket) {
    match packet {
        ClientPacket::CreateLobby { username, public } => {
            let lobby_id = Uuid::now_v7();
            let lobby = Lobby {
                public,
                players: HashMap::from_iter([(
                    client_id,
                    LobbyPlayer {
                        username,
                        ready: false,
                    },
                )]),
            };
            let mut lobbies = LOBBIES.write().await;
            lobbies.insert(lobby_id, lobby);
            CLIENTS
                .write()
                .await
                .get_mut(&client_id)
                .expect("client not found")
                .status = ClientStatus::InLobby(lobby_id);
            send_packet(client_id, ServerPacket::LobbyJoined(lobby_id)).await;
        }
        ClientPacket::JoinLobby { lobby_id, username } => {
            let mut lobbies = LOBBIES.write().await;

            let (lobby_id, lobby) = match lobby_id {
                Some(id) => {
                    let Some(lobby) = lobbies.get_mut(&id) else {
                        return send_packet(
                            client_id,
                            ServerPacket::Refused("lobby not found".to_string()),
                        )
                        .await;
                    };
                    (id, lobby)
                }
                None => {
                    let random_lobby = lobbies
                        .iter_mut()
                        .filter(|(_, lobby)| lobby.public)
                        .min_by_key(|(_, lobby)| lobby.players.len());
                    match random_lobby {
                        Some((&id, lobby)) => (id, lobby),
                        None => {
                            let id = Uuid::now_v7();
                            (
                                id,
                                lobbies.entry(id).or_insert(Lobby {
                                    public: true,
                                    players: HashMap::new(),
                                }),
                            )
                        }
                    }
                }
            };

            lobby.players.insert(
                client_id,
                LobbyPlayer {
                    username,
                    ready: false,
                },
            );

            CLIENTS
                .write()
                .await
                .get_mut(&client_id)
                .expect("client not found")
                .status = ClientStatus::InLobby(lobby_id);
            send_packet(client_id, ServerPacket::LobbyJoined(lobby_id)).await;

            let message = bincode::serialize(&lobby).expect("failed to serialize lobby");
            for player_id in lobby.players.keys() {
                send_message(*player_id, &message).await;
            }
        }
        _ => (),
    }
}

async fn handle_in_lobby(client_id: Uuid, lobby_id: Uuid, packet: ClientPacket) {
    match packet {
        ClientPacket::Disconnect => todo!(),
        ClientPacket::IAmReady => todo!(),
        ClientPacket::IAmNotReady => todo!(),
        _ => (),
    }
}

async fn handle_in_game(client_id: Uuid, game_id: Uuid, packet: ClientPacket) {}
