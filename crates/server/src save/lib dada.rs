use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerSecret(Uuid);

#[derive(Serialize, Deserialize)]
pub struct PlayerProfile {
    pub username: String,
    pub image_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub enum LoginRequest {
    Create {
        username: String,
    },
    JoinRandom {
        username: String,
    },
    Join {
        game_id: GameId,
        username: String,
    },
    Rejoin {
        game_id: GameId,
        player_id: PlayerId,
        secret: PlayerSecret,
    },
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse<T> {
    Refused(String),
    Success(T),
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    game_id: GameId,
    player_id: PlayerId,
    secret: PlayerSecret,
    options: Vec<GameSettingField>,
}

#[derive(Serialize, Deserialize)]
pub struct GameSettingField {
    pub name: String,
    pub description: String,
    pub field_type: GameSettingFieldType,
}

#[derive(Serialize, Deserialize)]
pub enum GameSettingFieldType {
    Integer { min: i32, max: i32 },
    Decimal { min: f32, max: f32 },
    String { min_len: usize, max_len: usize },
    Choice { choices: HashSet<String> },
    Boolean,
}

#[derive(Serialize, Deserialize)]
pub enum GameSettingFieldValue {
    Integer(i32),
    Decimal(f32),
    String(String),
    Choice(String),
    Boolean(bool),
}

#[derive(Serialize, Deserialize)]
pub struct JoinRandomResponse {
    game_id: GameId,
    player_id: PlayerId,
    secret: PlayerSecret,
    players: HashMap<PlayerId, PlayerProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinResponse {
    player_id: PlayerId,
    secret: PlayerSecret,
    players: HashMap<PlayerId, PlayerProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct RejoinResponse {
    players: HashMap<PlayerId, PlayerProfile>,
}
