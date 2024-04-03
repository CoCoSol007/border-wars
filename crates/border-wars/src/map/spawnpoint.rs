//! TODO

use bevnet::Connection;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy::utils::HashMap;

use super::generation::EndMapGeneration;
use super::TilePosition;
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
    players: Query<(Entity, &Player)>,
    map: Query<(Entity, &TilePosition)>,
    connection: Res<Connection>,
) {
    if end_map_event.is_empty() {
        return;
    }

    let radius = map
        .iter()
        .max_by(|(_, a), (_, b)| a.0.cmp(&b.0))
        .unwrap()
        .1
        .0
        .abs();

    if radius == 0 {
        panic!("Map radius must be greater than 0");
    }

    let map_hashmap: HashMap<&TilePosition, Entity> = map.iter().map(|(e, p)| (p, e)).collect();

    let nb_player = players.iter().count();

    let mut spawnpoints = Vec::with_capacity(nb_player);

    for (i, target_position) in TilePosition::new(0, 0)
        .ring(radius as usize / 2)
        .enumerate()
    {
        let Some(target_entity) = map_hashmap.get(&target_position) else {
            return;
        };

        if i % (radius as usize * 3 / nb_player) != 0 {
            continue;
        }
        spawnpoints.push((*target_entity, target_position));
    }

    let mut sorted_players = players.iter().collect::<Vec<_>>();
    sorted_players.sort_by(compare_player);
    spawnpoints.sort_by(compare_spawnpoint_entity);

    for (i, (target_entity, target_position)) in spawnpoints.iter().enumerate() {
        let player = sorted_players[i].1;
        if Some(player.uuid) == connection.identifier() {
            commands.entity(*target_entity).despawn();
        }
    }
}

/// TODO
fn compare_player((_, a): &(Entity, &Player), (_, b): &(Entity, &Player)) -> std::cmp::Ordering {
    a.uuid.cmp(&b.uuid)
}

/// TODO
fn compare_spawnpoint_entity(
    (_, a): &(Entity, TilePosition),
    (_, b): &(Entity, TilePosition),
) -> std::cmp::Ordering {
    let r = a.0.abs().cmp(&b.0.abs());
    if r == std::cmp::Ordering::Equal {
        a.1.cmp(&b.1)
    } else {
        r
    }
}
