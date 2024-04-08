//! Raft implementation for the relay client.

use std::borrow::Cow;
use std::collections::{BTreeSet, LinkedList};
use std::io;

use prost::bytes::BufMut;
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
    connection: Connection,
    node: RaftNode<RaftLogMemory, ChaChaRng, Uuid>,
    peers: BTreeSet<Uuid>,
}

impl RaftConnection {
    /// Creates a new Raft connection from a current connection.
    /// Returns an error if the connection does not have an identifier.
    pub fn from(connection: Connection, peers: BTreeSet<Uuid>) -> Result<Self, Connection> {
        let Some(identifier) = connection.identifier() else {
            return Err(connection);
        };
        Ok(Self {
            connection,
            node: RaftNode::new(
                identifier,
                peers.clone(),
                RaftLogMemory::new_unbounded(),
                ChaChaRng::seed_from_u64(identifier.as_u64_pair().0),
                RaftConfig {
                    election_timeout_ticks: 10,
                    heartbeat_interval_ticks: 1,
                    replication_chunk_size: usize::max_value(),
                },
            ),
            peers,
        })
    }

    /// Envoit un message à tous les noeuds du cluster.
    pub fn append<'a>(&mut self, message: impl Into<Cow<'a, [u8]>>) {
        let mut data = message.into().into_owned();
        if self.node.is_leader() {
            let Ok(messages) = self.node.append(data) else {
                panic!("OOOOOOOOH!");
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

            match sendable.dest {
                RaftMessageDestination::Broadcast => {
                    for peer in peers
                        .iter()
                        .filter(|&peer| Some(*peer) != connection.identifier())
                    {
                        connection.send(*peer, &data);
                    }
                }
                RaftMessageDestination::To(target) => connection.send(target, data),
            }
        }
    }

    pub fn update(&mut self) -> LinkedList<Vec<u8>> {
        let messages = self.node.timer_tick();
        Self::send_raft_messages(&self.connection, &self.peers, messages);

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
                        panic!("OOOOOOOOH!");
                    };
                    Self::send_raft_messages(&self.connection, &self.peers, messages);
                }
                _ => panic!("AAAAAH!"),
            }
        }

        let mut result = LinkedList::new();
        for message in self.node.take_committed() {
            result.push_back(message.data.to_vec());
        }
        result
    }
}
