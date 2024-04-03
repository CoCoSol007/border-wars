//! TODO

use bevnet::Connection;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy::utils::HashMap;

use super::generation::EndMapGeneration;
use super::ownership::Owner;
use super::{Tile, TilePosition};
use crate::Player;

/// The plugin for the spawn point.
pub struct SpawnPointPlugin;

impl Plugin for SpawnPointPlugin {
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
    connection: Res<Connection>,
) {
    for _ in end_map_event.iter() {
        let Some(radius) = map.iter().map(|(_, p, _)| p.0.abs()).max() else {
            return;
        };

        if radius == 0 {
            return;
        }

        let ring: HashSet<TilePosition> =
            TilePosition::new(0, 0).ring(radius as usize / 2).collect();

        let mut sorted_tiles = map
            .iter_mut()
            .filter(|(_, p, _)| ring.contains(*p))
            .collect::<Vec<_>>();

        sorted_tiles.sort_by(|a, b| compare_spawnpoint_entity(a.1, b.1));

        let mut sorted_players = players.iter().collect::<Vec<_>>();
        sorted_players.sort_by(|a: &&Player, b: &&Player| compare_player(a, b));

        for (i, tile) in sorted_tiles.iter_mut().enumerate() {
            let Some(player) = sorted_players.get(i) else {
                continue;
            };
            println!("{:?}", player);
            *tile.2 = Tile::Castle;
            commands.entity(tile.0).insert(Owner(Player::clone(player)));
        }
    }
}

/// TODO
fn compare_player(a: &Player, b: &Player) -> std::cmp::Ordering {
    a.uuid.cmp(&b.uuid)
}

/// TODO
fn compare_spawnpoint_entity(a: &TilePosition, b: &TilePosition) -> std::cmp::Ordering {
    let r = a.0.abs().cmp(&b.0.abs());
    if r == std::cmp::Ordering::Equal {
        a.1.cmp(&b.1)
    } else {
        r
    }
}
