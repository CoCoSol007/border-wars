//! A relay server for bevnet.

use std::io;

use anyhow::bail;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use axum::Router;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use sled::transaction::{ConflictableTransactionResult, TransactionalTree};
use sled::{Db, IVec};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

lazy_static! {
    static ref CLIENTS: DashMap<Uuid, Sender<Vec<u8>>> = DashMap::new();
    static ref DB: Db = sled::open("/data/secrets.db").expect("unable to open the database");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|ws: WebSocketUpgrade| async {
            ws.on_upgrade(|socket| async {
                handle(socket).await.ok();
            })
        }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("failed to serve");
}

/// Create a new client and add it to the database.
fn create_client(tx: &TransactionalTree) -> ConflictableTransactionResult<(Uuid, Uuid), io::Error> {
    // Generates a new identifier for the client.
    let client_id = loop {
        // Generates a new random identifier.
        let id = Uuid::new_v4();

        // Check if the id isn't already in the database.
        if tx.get(id.as_bytes())?.is_none() {
            break id;
        }
    };

    // Generate a random secret for the client.
    let secret = Uuid::new_v4();

    // Add the new client to the database.
    tx.insert(client_id.as_bytes(), secret.as_bytes())?;

    // Returns the client identifier and his secret.
    Ok((client_id, secret))
}

/// Handle the websocket connection.
async fn handle(mut socket: WebSocket) -> anyhow::Result<()> {
    // Receive the first request from the client.
    let data = match socket.recv().await {
        Some(Ok(message)) => message.into_data(),
        _ => return Ok(()),
    };

    // If the request is empty it means that the client want a new identifier and
    // secret, so we create them and send them to the client.
    let client_id = if data.is_empty() {
        // Generate the new client.
        let (client_id, secret) = DB.transaction(create_client)?;
        DB.flush_async().await?;
        println!("{client_id} created");

        // Send the data to the client.
        let mut data = Vec::with_capacity(32);
        data.extend_from_slice(client_id.as_bytes());
        data.extend_from_slice(secret.as_bytes());
        socket.send(Message::Binary(data)).await?;

        // Returns the client identifier.
        client_id
    }
    // Otherwise it means that the client want to reuse an identifier, so it will
    // send it along with his secret to prove that he is the right client.
    else {
        // Check for the message length to detect malformed messages.
        if data.len() != 32 {
            bail!("malformed message");
        }

        // Get the client identifier and secret from the message.
        let client_id = Uuid::from_slice(&data[..16])?;
        let secret = Uuid::from_slice(&data[16..])?;

        // Check with the database if the secret is correct.
        if DB.get(client_id.as_bytes())? != Some(IVec::from(secret.as_bytes())) {
            bail!("invalid secret")
        }

        // Returns the client identifier.
        client_id
    };

    // Handle the client connection.
    println!("{client_id} connected");
    let (sender, receiver) = channel(128);
    CLIENTS.insert(client_id, sender);
    handle_client(socket, client_id, receiver).await.ok();
    CLIENTS.remove(&client_id);
    println!("{client_id} disconnected");

    // Returns success.
    Ok(())
}

/// Handle the client connection.
async fn handle_client(
    socket: WebSocket,
    client_id: Uuid,
    mut receiver: Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    // Split the socket into sender and receiver.
    let (mut writer, mut reader) = socket.split();

    // Handle sending messages to the client.
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            writer.send(Message::Binary(message)).await?;
        }
        Ok::<(), axum::Error>(())
    });

    // Handle messages from the client.
    while let Some(Ok(message)) = reader.next().await {
        // Get the target ID from the message.
        let mut data = message.into_data();
        if data.len() < 16 {
            bail!("malformed message");
        }
        let id_start = data.len() - 16;
        let target_id = Uuid::from_slice(&data[id_start..])?;

        // Write the sender ID to the message.
        for (i, &byte) in client_id.as_bytes().iter().enumerate() {
            data[id_start + i] = byte;
        }

        // Send the message to the target client.
        if let Some(sender) = CLIENTS.get(&target_id) {
            sender.send(data).await?;
        }
    }

    // Returns success.
    Ok(())
}
