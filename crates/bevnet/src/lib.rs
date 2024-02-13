//! A networking library for Bevy.

use std::borrow::Cow;
use std::collections::LinkedList;

use bevy::prelude::*;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub use uuid::Uuid;

/// A connection to a relay server.
#[derive(Resource)]
pub struct Connection(relay_client::Connection);

/// A resource that stores the received messages.
#[derive(Resource)]
pub struct ReceivedMessages(DashMap<u16, LinkedList<(Uuid, Vec<u8>)>>);

impl Connection {
    /// Returns the identifier of the connection.
    pub const fn identifier(&self) -> Option<Uuid> {
        self.0.identifier()
    }
}

/// A bevy plugin to make multiplayer game using a relay server.
pub struct NetworkPlugin(String);

impl NetworkPlugin {
    /// Create a new [NetworkPlugin] plugin with the given domain for the relay
    /// server.
    pub fn new<'a>(domain: impl Into<Cow<'a, str>>) -> Self {
        Self(domain.into().into_owned())
    }
}

/// Update the relay connection.
fn update_connection(mut connection: ResMut<Connection>, received_messages: Res<ReceivedMessages>) {
    let messages = connection.0.update();
    for (sender, mut message) in messages {
        if message.len() < 2 {
            error!("message too short received");
            continue;
        }
        let id_start = message.len() - 2;
        let event_id = u16::from_be_bytes([message[id_start], message[id_start + 1]]);
        message.truncate(id_start);
        received_messages
            .0
            .entry(event_id)
            .or_default()
            .push_back((sender, message));
    }
}

/// A system that clear the received messages.
fn clear_received_messages(received_messages: Res<ReceivedMessages>) {
    received_messages.0.clear();
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Connection(
            relay_client::Connection::new(&self.0).expect("could not create connection"),
        ))
        .insert_resource(ReceivedMessages(DashMap::new()))
        .add_systems(PreUpdate, update_connection)
        .add_systems(PreUpdate, clear_received_messages.after(update_connection));
    }
}

/// A resource that store the last event id used to register an [Event].
///
/// This is used to give an unique id to each event.
#[derive(Resource, Default)]
struct LastEventId(u16);

/// An [Event] used to send an [Event] to another client on the relay server.
#[derive(Event)]
pub struct SendTo<T: Event + DeserializeOwned + Serialize>(pub Uuid, pub T);

/// An [Event] used to receive an [Event] from another client on the relay
/// server.
#[derive(Event)]
pub struct Receive<T: Event + DeserializeOwned + Serialize>(pub Uuid, pub T);

/// A trait that extends a bevy [App] to add multiplayer support.
pub trait NetworkAppExt {
    /// Setup the application to manage network events of type `T`.
    fn add_network_event<T: Event + DeserializeOwned + Serialize>(&mut self) -> &mut Self;
}

impl NetworkAppExt for App {
    fn add_network_event<T: Event + DeserializeOwned + Serialize>(&mut self) -> &mut Self {
        // Get a new event id.
        let mut event_id = self.world.get_resource_or_insert_with(LastEventId::default);
        event_id.0 += 1;
        let event_id = event_id.0;

        // Register the event.
        self.add_event::<SendTo<T>>()
            .add_event::<Receive<T>>()
            .add_systems(
                PreUpdate,
                (move |mut events: EventReader<SendTo<T>>, connection: Res<Connection>| {
                    for event in events.read() {
                        // Get the size of the serialized event.
                        let size = match bincode::serialized_size(&event.1) {
                            Ok(size) => size,
                            Err(e) => {
                                error!("failed to serialize event: {}", e);
                                continue;
                            }
                        };

                        // Serialize the event we add 18 here because we will add the event id (2
                        // bytes) at the end and after that, the relay client will add the target id
                        // at the end (16 bytes).
                        let mut data = Vec::with_capacity(size as usize + 18);
                        if let Err(e) = bincode::serialize_into(&mut data, &event.1) {
                            error!("failed to serialize event: {}", e);
                            continue;
                        }

                        // Add the event id.
                        data.extend_from_slice(&event_id.to_be_bytes());

                        // Send the event.
                        connection.0.send(event.0, data);
                    }
                })
                .before(update_connection),
            )
            .add_systems(
                PreUpdate,
                (move |mut writer: EventWriter<Receive<T>>,
                       received_messages: Res<ReceivedMessages>| {
                    if let Some(mut messages) = received_messages.0.get_mut(&event_id) {
                        while let Some((sender, message)) = messages.pop_front() {
                            match bincode::deserialize(&message) {
                                Ok(event) => writer.send(Receive(sender, event)),
                                Err(e) => error!("failed to deserialize event: {}", e),
                            }
                        }
                    }
                })
                .before(clear_received_messages)
                .after(update_connection),
            )
    }
}
