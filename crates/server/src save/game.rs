use dashmap::DashMap;
use lazy_static::lazy_static;
use server::SlotDashMap;
use slotmap::SlotMap;
use uuid::Uuid;

server::new_key_type! {
    struct GameId;
}

lazy_static! {
    static ref GAMES: SlotDashMap<GameId, Game> = SlotDashMap::new();
}

slotmap::new_key_type! {
    struct PlayerId;
}

pub struct Game {
    players: SlotMap<PlayerId, Player>,
}

pub struct Player {
    connection_secret: Uuid,
}
