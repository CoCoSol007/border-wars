use std::collections::HashMap;

use anyhow::{bail, Context};
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use server::{GameId, Lobby, LobbyId, LoginRequest, PlayerId};
use uuid::Uuid;

lazy_static! {
    static ref LOBBIES: DashMap<LobbyId, Lobby> = DashMap::new();
    // static ref GAMES: DashMap<GameId, Game> = DashMap::new();
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
    let Message::Binary(login_data) = socket.recv().await.context("client disconnected")?? else {
        bail!("expected login request");
    };
    let login_request = bincode::deserialize(&login_data)?;
    match login_request {
        LoginRequest::CreateLobby { username, public } => {
            let lobby_id = loop {
                let id = Uuid::new_v4();
                let Entry::Vacant(entry) = LOBBIES.entry(LobbyId(id)) else {
                    continue;
                };
                entry.insert()
            }
        }
        LoginRequest::JoinLobby { lobby_id, username } => todo!(),
        LoginRequest::RejoinGame { token } => todo!(),
    }

    Ok(())
}

async fn handle_game_creation(mut socket: WebSocket, username: String) -> anyhow::Result<()> {
    Ok(())
}
