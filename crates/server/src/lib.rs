use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ClientPacket {
    Disconnect,
    CreateLobby {
        username: String,
        public: bool,
    },
    JoinLobby {
        lobby_id: Option<Uuid>,
        username: String,
    },
    IAmReady,
    IAmNotReady,
}

#[derive(Serialize, Deserialize)]
pub enum ServerPacket {
    Refused(String),
    LobbyJoined(Uuid),
    LobbyUpdated(Lobby),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Lobby {
    pub public: bool,
    pub players: HashMap<Uuid, LobbyPlayer>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyPlayer {
    pub username: String,
    pub ready: bool,
}
