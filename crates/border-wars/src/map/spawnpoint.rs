//! All code related to set the spawn point of the players.

use bevy::prelude::*;

use super::generation::EndMapGeneration;
use super::ownership::Owner;
use super::{Tile, TilePosition};
use crate::Player;

/// The plugin that initialize the spawn point at the start of the game.
pub struct SpawnpointPlugin;

impl Plugin for SpawnpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_spawn_point);
    }
}

/// Initialize the spawn point when the map is generated.
fn init_spawn_point(
    mut commands: Commands,
    mut end_map_event: EventReader<EndMapGeneration>,
    players: Query<&Player>,
    mut map: Query<(Entity, &TilePosition, &mut Tile)>,
) {
    for _ in end_map_event.read() {
        // Calculate the radius of the map.
        let Some(radius) = map.iter().map(|(_, p, _)| p.0.abs()).max() else {
            warn!("The map is empty");
            return;
        };

        if radius == 0 {
            warn!("The map is empty");
            return;
        }

        let mut sorted_players = players.iter().collect::<Vec<_>>();
        sorted_players.sort_by(|a: &&Player, b: &&Player| a.uuid.cmp(&b.uuid));

        let mut sorted_players = sorted_players.iter();

        let interval = radius as usize * 3 / sorted_players.len();

        for (i, position) in TilePosition::new(0, 0)
            .ring(radius as usize / 2)
            .enumerate()
        {
            // Find the target tile.
            let Some((entity, _, mut tile)) = map.iter_mut().find(|(_, p, _)| **p == position)
            else {
                continue;
            };

            // Check the interval between players.
            if i % interval != 0 {
                continue;
            }

            // Get the current player.
            let Some(player) = sorted_players.next() else {
                continue;
            };

            // Set the spawn point.
            *tile = Tile::Castle;
            commands.entity(entity).insert(Owner(Player::clone(player)));
        }
    }
}
