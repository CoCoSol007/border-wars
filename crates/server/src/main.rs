use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use ascon_hash::{AsconXof, ExtendableOutput, Update, XofReader};
use axum::extract::ws::Message;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use manager::handle_event;
use server::{ClientMessage, ServerMessage};
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::RwLock;
use uuid::Uuid;

mod manager;

lazy_static! {
    pub static ref CONNECTIONS: ConnectionManager = ConnectionManager::default();
}

#[derive(Default)]
pub struct ConnectionManager(RwLock<HashMap<Uuid, Sender<Vec<u8>>>>);

impl ConnectionManager {
    pub async fn send(&self, id: Uuid, message: impl Borrow<ServerMessage>) {
        if let Some(sender) = self.0.read().await.get(&id) {
            let Ok(message) = bincode::serialize(message.borrow()) else {
                eprintln!("failed to serialize message");
                return;
            };
            sender.send(message).await.ok();
        }
    }
}

pub enum ClientEvent {
    Connected,
    Message(ClientMessage),
    Disconnected,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
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

        // Start the sending loop
        let (mut writer, mut reader) = socket.split();
        let (sender, mut receiver) = channel(1);
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                writer.send(Message::Binary(message)).await?;
            }
            Ok::<(), axum::Error>(())
        });

        // Register the client
        match CONNECTIONS.0.write().await.entry(id) {
            Entry::Occupied(_) => return,
            Entry::Vacant(entry) => {
                entry.insert(sender);
            }
        }

        // Send the connection event
        handle_event(id, ClientEvent::Connected).await;

        // Handle incoming messages
        while let Some(Ok(message)) = reader.next().await {
            let Ok(message) = bincode::deserialize(&message.into_data()) else {
                continue;
            };
            handle_event(id, ClientEvent::Message(message)).await;
        }

        // Send the disconnection event
        handle_event(id, ClientEvent::Disconnected).await;

        // Unregister the client
        CONNECTIONS.0.write().await.remove(&id);
    })
}
