use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    CreateLobby {
        username: String,
        public: bool,
    },
    JoinLobby {
        username: String,
        lobby_id: Option<Uuid>,
    },
    Ready(bool),
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Refused(String),
    LobbyUpdate(Lobby),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Lobby {
    pub id: Uuid,
    pub public: bool,
    pub players: HashMap<Uuid, Player>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub ready: bool,
}
