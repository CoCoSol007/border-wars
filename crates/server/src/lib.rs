use std::collections::HashSet;

use uuid::Uuid;

struct Action {
    title: String,
    description: String,
    tile_id: Uuid,
    image: 
}

struct ActionType {}

struct Player {}

impl Player {
    async fn make_action(&mut self, actions: HashSet<Action>) -> Action {
        todo!()
    }
}

async fn border_wars_classic(players: HashSet<Player>) {
    let player1 = players.iter().next().unwrap();

    let action = player1.make_action(HashSet::new()).await;
}
