use std::collections::LinkedList;

use anyhow::{bail, Context};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

slotmap::new_key_type! {
    struct PlayerId;
}

enum WorldModification {
    SetTile { r: i32, s: i32, image_id: Uuid },
}

struct Player {
    connection_secret: Uuid,
    unsent_modifications: LinkedList<WorldModification>,
}

pub struct Game {
    players: SlotMap<PlayerId, Player>,
}

trait Request: DeserializeOwned + Serialize {
    type Response: Response;
}

trait Response: DeserializeOwned + Serialize {}

lazy_static! {
    static ref CLIENTS: DashMap<Uuid, Sender<Vec<u8>>> = DashMap::new();
    static ref GAMES: DashMap<Uuid, Game> = DashMap::new();
}

#[derive(Serialize, Deserialize)]
pub enum LoginRequest {
    Create { pseudo: String },
    JoinRandom { pseudo: String },
    Join { game_id: Uuid, pseudo: String },
    Rejoin { game_id: Uuid },
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async {
            ws.on_upgrade(|socket| async {
                if let Err(e) = handle(socket).await {
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

async fn handle(mut socket: WebSocket) -> anyhow::Result<()> {
    let Message::Binary(data) = socket
        .recv()
        .await
        .context("client disconnected before login")??
    else {
        bail!("expected login request");
    };

    let login_request = bincode::deserialize(&data)?;
    match login_request {
        LoginRequest::Create { pseudo } => todo!(),
        LoginRequest::JoinRandom { pseudo } => todo!(),
        LoginRequest::Join { game_id, pseudo } => todo!(),
        LoginRequest::Rejoin { game_id } => todo!(),
    }

    let (mut writer, mut reader) = socket.split();

    let (global_sender, mut receiver) = channel(128);
    let client_id = loop {
        let id = Uuid::new_v4();
        let Entry::Vacant(entry) = CLIENTS.entry(id) else {
            continue;
        };
        entry.insert(global_sender);
        break id;
    };

    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            writer.send(Message::Binary(message)).await?;
        }
        Ok::<(), axum::Error>(())
    });

    while let Some(Ok(message)) = reader.next().await {
        let Message::Binary(data) = message else {
            continue;
        };

        todo!()
    }

    CLIENTS.remove(&client_id);

    Ok(())
}

async fn send_message(id: Uuid, message: Vec<u8>) -> bool {
    if let Some(sender) = CLIENTS.get(&id) {
        sender.send(message).await.is_ok()
    } else {
        false
    }
}
