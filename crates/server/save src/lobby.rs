use std::collections::HashMap;

use uuid::Uuid;

pub struct LobbyManager {
    connections: HashMap<Uuid, Uuid>,
    lobbies: HashMap<Uuid, Lobby>,
}

pub struct Lobby {
    id: Uuid,
    connections: HashMap<Uuid, Uuid>,
}
