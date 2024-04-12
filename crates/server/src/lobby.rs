use axum::http::header::Entry;
use dashmap::DashMap;
use lazy_static::lazy_static;
use scc::{HashMap, HashSet};
use server::LobbyStatus;
use uuid::Uuid;

lazy_static! {
    static ref LOBBIES: DashMap<Uuid, LobbyStatus> = DashMap::new();
}

pub(crate) async fn create_lobby(
    client_id: Uuid,
    username: String,
    public: bool,
) -> anyhow::Result<()> {
    todo!()
}
