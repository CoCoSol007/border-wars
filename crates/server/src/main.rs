use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::mpsc::{channel, Sender};
use uuid::Uuid;

trait Request: DeserializeOwned + Serialize {
    type Response: Response;
}

trait Response: DeserializeOwned + Serialize {}

lazy_static! {
    static ref CLIENTS: DashMap<Uuid, Sender<Vec<u8>>> = DashMap::new();
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async { ws.on_upgrade(|socket| handle(socket)) }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

async fn handle(mut socket: WebSocket) {
    let (mut writer, mut reader) = socket.split();

    let (sender, mut receiver) = channel(128);
    let client_id = loop {
        let id = Uuid::new_v4();
        let Entry::Vacant(entry) = CLIENTS.entry(id) else {
            continue;
        };
        entry.insert(sender);
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
        if let Err(error) = message_received(client_id, data).await {
            println!("Error: {}", error);
            break;
        };
    }

    CLIENTS.remove(&client_id);
}

async fn send_message(id: Uuid, message: Vec<u8>) -> anyhow::Result<()> {
    if let Some(sender) = CLIENTS.get(&id) {
        sender.send(message).await?;
    }
    Ok(())
}

async fn message_received(id: Uuid, message: Vec<u8>) -> anyhow::Result<()> {
    Ok(())
}
