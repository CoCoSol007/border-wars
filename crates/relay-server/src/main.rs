//! A relay server for bevnet.

use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use rand::Rng;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

lazy_static! {
    static ref CLIENTS: DashMap<u32, Sender<Vec<u8>>> = DashMap::new();
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async { ws.on_upgrade(handle) }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

/// Handle the websocket connection.
async fn handle(socket: WebSocket) {
    // Generate a new ID for the client.
    let client_id: u32 = loop {
        let id = rand::thread_rng().gen();
        if !CLIENTS.contains_key(&id) {
            break id;
        }
    };
    println!("Client({}) connected", client_id);

    // Add the client to the list of connected clients.
    let (sender, receiver) = channel(128);
    CLIENTS.insert(client_id, sender);

    // Handle messages from the client.
    let result = handle_socket(socket, client_id, receiver).await;

    // Remove the client from the list of connected clients.
    match result {
        Ok(_) => println!("Client({}) disconnected", client_id),
        Err(e) => {
            CLIENTS.remove(&client_id);
            println!("Client({}) disconnected: {}", client_id, e);
        }
    }
}

/// Error prone part of handling the websocket connection.
async fn handle_socket(
    mut socket: WebSocket,
    client_id: u32,
    mut receiver: Receiver<Vec<u8>>,
) -> Result<(), axum::Error> {
    // Send the client ID to the client.
    socket
        .send(Message::Binary(client_id.to_be_bytes().to_vec()))
        .await?;

    // Split the socket into sender and receiver.
    let (mut writer, mut reader) = socket.split();

    // Handle sending messages to the client.
    let sending_task: JoinHandle<Result<(), axum::Error>> = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            writer
                .send(Message::Binary(message))
                .await
                .map_err(axum::Error::new)?;
        }
        Ok(())
    });

    // Handle messages from the client.
    while let Some(Ok(message)) = reader.next().await {
        // Get the target ID from the message.
        let mut data = message.into_data();
        let id_start = data.len() - 4;
        let target_id = u32::from_be_bytes(data[id_start..].try_into().map_err(axum::Error::new)?);

        // Write the sender ID to the message.
        for (i, byte) in client_id.to_be_bytes().into_iter().enumerate() {
            data[id_start + i] = byte;
        }

        // Send the message to the target client.
        if let Some(sender) = CLIENTS.get(&target_id) {
            sender.send(data).await.map_err(axum::Error::new)?;
        }
    }

    // Remove the client from the list of connected clients.
    CLIENTS.remove(&client_id);

    // Wait for the sender to finish.
    sending_task.await.map_err(axum::Error::new)?
}
