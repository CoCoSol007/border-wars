//! TODO

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

    let map_hashmap: HashMap<&TilePosition, Entity> = map.iter().map(|(e, p)| (p, e)).collect();

    let nb_player = players.iter().count();
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
        commands.entity(*target_entity).despawn();
    }
}
