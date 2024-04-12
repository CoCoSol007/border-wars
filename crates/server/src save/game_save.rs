use std::collections::{HashSet, LinkedList};
use std::time::Duration;

use rand::Rng;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use uuid::Uuid;

use crate::{receive_message, send_message};

slotmap::new_key_type! {
    struct ActionId;
}

#[derive(Serialize, Deserialize)]
struct Action {
    title: String,
    description: String,
    next_actions: SlotMap<ActionId, Self>,
    tile_position: Option<(i32, i32)>,
    image_id: Uuid,
}

struct Player {
    connection: Option<Uuid>,
    secret: Uuid,
}

impl Player {
    fn get_random_action(actions: &SlotMap<ActionId, Action>) -> LinkedList<ActionId> {
        if actions.len() == 0 {
            return LinkedList::new();
        }
        let random_index = rand::thread_rng().gen_range(0..actions.len());
        let action = actions.iter().nth(random_index).unwrap();
        let mut result = LinkedList::new();
        result.push_back(action.0);
        result.append(&mut Self::get_random_action(&action.1.next_actions));
        result
    }

    async fn make_action(
        &mut self,
        actions: &SlotMap<ActionId, Action>,
        timeout: Duration,
    ) -> LinkedList<ActionId> {
        let Some(connection) = self.connection else {
            return Self::get_random_action(actions);
        };

        let Ok(encoded_message) = bincode::serialize(actions) else {
            return Self::get_random_action(actions);
        };

        if !send_message(connection, encoded_message).await {
            return Self::get_random_action(actions);
        }

        // Get the client response
        match tokio::time::timeout(timeout, receive_message(connection)).await {
            Ok(Some(response)) => match bincode::deserialize(&response) {
                Ok(response) => response,
                Err(_) => Self::get_random_action(actions),
            },
            _ => Self::get_random_action(actions),
        }
    }
}
