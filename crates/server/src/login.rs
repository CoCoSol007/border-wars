use anyhow::{bail, Context};
use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum LoginRequest {
    Create { username: String },
}

pub async fn handle_client_login(socket: &mut WebSocket) -> anyhow::Result<()> {
    let Message::Binary(login_data) = socket.recv().await.context("client disconnected")?? else {
        bail!("expected login request");
    };

    let login_request = bincode::deserialize(&login_data)?;
    match login_request {
        LoginRequest::Create { username } => todo!(),
    }
}
