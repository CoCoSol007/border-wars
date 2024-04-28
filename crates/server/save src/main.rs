use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use ascon_hash::{AsconXof, ExtendableOutput, Update, XofReader};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::routing::get;
use axum::Router;
use futures::{SinkExt, StreamExt};
use server::JoinRequest;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::RwLock;
use uuid::Uuid;

mod lobby;

#[derive(Clone, Default)]
struct GameManager {
    lobbies: Arc<RwLock<HashMap<Uuid, HashMap<Uuid, Sender<Vec<u8>>>>>>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(GameManager::default());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    State(manager): State<GameManager>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        // Handle authentication
        let Some(Ok(message)) = socket.recv().await else {
            return;
        };
        let mut reader = AsconXof::default()
            .chain(message.into_data())
            .finalize_xof();
        let mut hash = [0u8; 16];
        reader.read(&mut hash);
        let id = Uuid::from_bytes(hash);

        // Get client request
        let Some(Ok(message)) = socket.recv().await else {
            return;
        };
        let Ok(JoinRequest(username, lobby_id)) = bincode::deserialize(&message.into_data()) else {
            return;
        };

        // Check if the client is in a game, if so, connect him to the game
        todo!();

        // Check if the client is already in a lobby, if so, refuse the connection
        let mut lobbies = manager.lobbies.write().await;
        for lobby in lobbies.values() {
            if lobby.contains_key(&id) {
                return;
            }
        }

        // Find or create the lobby
        let (lobby_id, lobby) = match lobby_id {
            Some(id) if id == Uuid::nil() => lobbies
                .iter_mut()
                .min_by_key(|(_, lobby)| lobby.len())
                .map(|(&id, lobby)| (id, lobby))
                .unwrap_or_else(|| {
                    let id = Uuid::now_v7();
                    (id, lobbies.entry(id).or_default())
                }),
            Some(id) => {
                let Some(lobby) = lobbies.get_mut(&id) else {
                    return;
                };
                (id, lobby)
            }
            None => {
                let id = Uuid::now_v7();
                (id, lobbies.entry(id).or_default())
            }
        };

        // Initialize the sending loop
        let (sender, mut receiver) = channel(1);
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                socket.send(Message::Binary(message)).await.ok();
            }
        });

        // Insert the client in the lobby
        lobby.insert(id, sender);
        drop(lobbies);

        // Wait for the client to be ready
        let Some(Ok(message)) = socket.recv().await else {
            return;
        };
    })
}
