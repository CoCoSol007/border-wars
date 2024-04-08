//! Raft implementation for the relay client.

use std::borrow::Cow;
use std::collections::{BTreeSet, LinkedList};

use prost::Message;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use relay_client::Connection;
use simple_raft::log::mem::RaftLogMemory;
use simple_raft::message::{RaftMessage, RaftMessageDestination, SendableRaftMessage};
use simple_raft::node::{RaftConfig, RaftNode};
use uuid::Uuid;

/// A Raft node.
pub struct RaftConnection {
    /// The connection to the relay server.
    connection: Connection,

    /// The Raft node.
    node: RaftNode<RaftLogMemory, ChaChaRng, Uuid>,

    /// The peers of the Raft cluster.
    peers: BTreeSet<Uuid>,
}

pub struct RaftConnectionConfig {
    /// The minimum number of timer ticks between leadership elections.
    pub election_timeout_ticks: u32,

    /// The number of timer ticks between sending heartbeats to peers.
    pub heartbeat_interval_ticks: u32,

    /// The maximum number of bytes to replicate to a peer at a time.
    pub replication_chunk_size: usize,
}

impl From<RaftConnectionConfig> for RaftConfig {
    fn from(val: RaftConnectionConfig) -> Self {
        RaftConfig {
            election_timeout_ticks: val.election_timeout_ticks,
            heartbeat_interval_ticks: val.heartbeat_interval_ticks,
            replication_chunk_size: val.replication_chunk_size,
        }
    }
}

impl RaftConnection {
    /// Creates a new Raft connection from a current connection.
    /// Returns an error if the connection does not have an identifier.
    pub fn from(
        connection: Connection,
        peers: BTreeSet<Uuid>,
        raft_config: RaftConnectionConfig,
    ) -> Result<Self, Connection> {
        let Some(identifier) = connection.identifier() else {
            return Err(connection);
        };

        let raft_node = Self {
            connection,
            node: RaftNode::new(
                identifier,
                peers.clone(),
                RaftLogMemory::new_unbounded(),
                ChaChaRng::seed_from_u64(identifier.as_u64_pair().0),
                raft_config.into(),
            ),
            peers,
        };

        Ok(raft_node)
    }

    /// Envoit un message à tous les noeuds du cluster.
    pub fn append<'a>(&mut self, message: impl Into<Cow<'a, [u8]>>) {
        let mut data = message.into().into_owned();
        if self.node.is_leader() {
            let Ok(messages) = self.node.append(data) else {
                panic!("Message just cancelled.");
            };
            Self::send_raft_messages(&self.connection, &self.peers, messages);
        } else {
            data.push(1);
            if let (Some(leader), _) = self.node.leader() {
                self.connection.send(*leader, data);
            }
        }
    }

    fn send_raft_messages(
        connection: &Connection,
        peers: &BTreeSet<Uuid>,
        sendables: impl Iterator<Item = SendableRaftMessage<Uuid>>,
    ) {
        for sendable in sendables {
            let mut data: Vec<u8> = Vec::with_capacity(sendable.message.encoded_len() + 19);
            sendable.message.encode(&mut data).ok();
            data.push(0);

            // Send the message to the target node.
            if let RaftMessageDestination::To(target) = sendable.dest {
                connection.send(target, data);
                return;
            }

            // Broadcast the message to all peers.
            peers
                .iter()
                .filter(|&peer| Some(*peer) != connection.identifier())
                .for_each(|peer| {
                    connection.send(*peer, &data);
                });
        }
    }

    pub fn update(&mut self) -> LinkedList<Vec<u8>> {
        // Update the Raft node.
        let messages = self.node.timer_tick();
        Self::send_raft_messages(&self.connection, &self.peers, messages);

        // Update the connection.
        let messages = self.connection.update();
        for (sender_id, mut message) in messages {
            let message_type = message[message.len() - 1];
            message.truncate(message.len() - 1);
            match message_type {
                0 => {
                    let message = RaftMessage::decode(&*message).unwrap();
                    let messages = self.node.receive(message, sender_id);
                    Self::send_raft_messages(&self.connection, &self.peers, messages);
                }
                1 if self.node.is_leader() => {
                    let Ok(messages) = self.node.append(message) else {
                        panic!("Message just cancelled.");
                    };
                    Self::send_raft_messages(&self.connection, &self.peers, messages);
                }
                _ => (),
            }
        }

        self.node
            .take_committed()
            .map(|v| v.data.to_vec())
            .collect()
    }
}
