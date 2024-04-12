use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LobbyId(Uuid);

#[derive(Serialize, Deserialize)]
pub struct Lobby {
    id: LobbyId,
    public: bool,
    players: HashMap<PlayerId, LobbyPlayer>,
}

#[derive(Serialize, Deserialize)]
pub struct LobbyPlayer {
    username: String,
    ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RejoinToken(Uuid);

#[derive(Serialize, Deserialize)]
pub enum LoginRequest {
    CreateLobby {
        username: String,
        public: bool,
    },
    JoinLobby {
        lobby_id: Option<Uuid>,
        username: String,
    },
    RejoinGame {
        token: RejoinToken,
    },
}

#[derive(Serialize, Deserialize)]
pub struct LobbyJoined {
    player_id: PlayerId,
    lobby: Lobby,
}

#[derive(Serialize, Deserialize)]
pub enum LobbyClientPacket {
    Ready(bool),
}

#[derive(Serialize, Deserialize)]
pub enum LobbyServerPacket {
    LobbyUpdated(Lobby),
    GameStarted(GameId, RejoinToken),
}

#[derive(Serialize, Deserialize)]
pub struct RejoinResponse;
