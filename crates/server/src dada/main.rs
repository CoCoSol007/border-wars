use std::collections::{HashMap, HashSet};
use std::hash::RandomState;
use std::sync::Arc;

use anyhow::{bail, Context};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::mapref::entry::Entry;
use dashmap::mapref::multiple::RefMutMulti;
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use server::GlobalRefMut;
use slotmap::SlotMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct ConnectionSender(Sender<Vec<u8>>);

impl ConnectionSender {
    pub async fn send<T: Serialize>(&mut self, message: T) -> anyhow::Result<()> {
        Ok(self.0.send(bincode::serialize(&message)?).await?)
    }
}

pub struct ConnectionReader(SplitStream<WebSocket>);

impl ConnectionReader {
    pub async fn read<T: DeserializeOwned>(&mut self) -> anyhow::Result<T> {
        loop {
            let Message::Binary(message) = self.0.next().await.context("no message")?? else {
                continue;
            };
            return Ok(bincode::deserialize(&message)?);
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async {
            ws.on_upgrade(|socket| async {
                let (mut sender, receiver) = socket.split();
                let (send_tx, mut send_rx) = channel(16);
                tokio::spawn(async move {
                    while let Some(message) = send_rx.recv().await {
                        sender.send(Message::Binary(message)).await?;
                    }
                    Ok::<(), axum::Error>(())
                });
                if let Err(e) = handle(ConnectionSender(send_tx), ConnectionReader(receiver)).await
                {
                    eprintln!("Error: {}", e);
                }
            })
        }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

lazy_static! {
    static ref LOBBIES: DashMap<Uuid, Lobby> = DashMap::new();
}

#[derive(Serialize, Deserialize)]
enum LoginRequest {
    CreateLobby {
        username: String,
        public: bool,
    },
    JoinLobby {
        lobby_id: Option<Uuid>,
        username: String,
    },
}

slotmap::new_key_type! {
struct ConnectionId;}

struct Lobby {
    id: Uuid,
    public: bool,
    connections: SlotMap<ConnectionId, LobbyConnection>,
}

struct LobbyConnection {
    sender: ConnectionSender,
    username: String,
    ready: bool,
}

#[derive(Serialize, Deserialize)]
struct LobbyJoined(Uuid);

#[derive(Serialize, Deserialize)]
struct IAmReady;

async fn handle(
    mut sender: ConnectionSender,
    mut receiver: ConnectionReader,
) -> anyhow::Result<()> {
    // Find or create a lobby
    let login_request: LoginRequest = receiver.read().await?;
    let (mut lobby, username) = match login_request {
        LoginRequest::CreateLobby { username, public } => (create_lobby(public).await, username),
        LoginRequest::JoinLobby { lobby_id, username } => (
            match lobby_id {
                Some(id) => LOBBIES.get_mut(&id).context("lobby not found")?.into(),
                None => match find_random_lobby().await {
                    Some(lobby) => lobby,
                    None => create_lobby(true).await,
                },
            },
            username,
        ),
    };

    // Add the user to the lobby
    let lobby_id = lobby.id;
    sender.send(LobbyJoined(lobby_id)).await?;
    let connection_id = lobby.connections.insert(LobbyConnection {
        sender,
        username,
        ready: false,
    });
    drop(lobby);

    // Wait for the user to be ready
    let disconnected = receiver.read::<IAmReady>().await.is_err();

    // Check to start the game
    let Entry::Occupied(mut lobby) = LOBBIES.entry(lobby_id) else {
        bail!("lobby not found");
    };
    if disconnected {
        lobby.get_mut().connections.remove(connection_id);
    }

    if lobby.get().connections.is_empty() {
        LOBBIES.remove(&lobby_id);
        return Ok(());
    }
    let should_start = lobby.connections.iter().all(|(_, c)| c.ready);

    todo!()
}

async fn create_lobby(public: bool) -> GlobalRefMut<'static, Uuid, Lobby> {
    loop {
        let id = Uuid::new_v4();
        if let Entry::Vacant(e) = LOBBIES.entry(id) {
            break e
                .insert(Lobby {
                    id,
                    public,
                    connections: SlotMap::with_key(),
                })
                .into();
        }
    }
}

async fn find_random_lobby() -> Option<GlobalRefMut<'static, Uuid, Lobby>> {
    LOBBIES
        .iter_mut()
        .filter(|lobby| lobby.public)
        .min_by_key(|lobby| lobby.connections.len())
        .map(GlobalRefMut::from)
}
