use std::collections::BTreeSet;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{io, thread};

use relay_client::Connection;
use relay_raft::{RaftConnection, RaftConnectionConfig};
use uuid::Uuid;

fn main() {
    let mut connection = Connection::new("relay.cocosol.fr").unwrap();
    while connection.identifier().is_none() {
        connection.update();
        thread::sleep(Duration::from_millis(100));
    }
    let identifier = connection.identifier().unwrap();
    println!("Identifier: {}", identifier);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let peers: BTreeSet<Uuid> = user_input
        .trim()
        .split(',')
        .map(|s| Uuid::parse_str(s).unwrap())
        .collect();

    let Ok(mut connection) = RaftConnection::from(
        connection,
        peers,
        RaftConnectionConfig {
            election_timeout_ticks: 10,
            heartbeat_interval_ticks: 1,
            replication_chunk_size: usize::max_value(),
        },
    ) else {
        panic!("Failed to create raft connection");
    };

    let (sender, receiver) = channel();
    thread::spawn(move || {
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).unwrap();
            sender.send(message.replace(['\n', '\r'], "")).unwrap();
        }
    });

    loop {
        if let Ok(message) = receiver.try_recv() {
            connection.append(message.as_bytes());
        }
        let messages = connection.update();

        for message in messages {
            println!("Received message: {}", String::from_utf8_lossy(&message));
        }

        thread::sleep(Duration::from_millis(100));
    }
}
