//! All functions related to the generation of the map.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use super::hex::*;
use super::{Tile, TilePosition};

/// A plugin to handle the map generation.
pub struct MapGenerationPlugin;

/// The zoom of the map during the generation.
const MAP_GENERATION_SCALE: f32 = 5.;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMapGeneration>()
            .add_event::<EndMapGeneration>()
            .add_systems(
                Update,
                (delete_map, generate_map.after(delete_map))
                    .run_if(in_state(crate::CurrentScene::Game)),
            );
    }
}

/// An event to trigger the generation of the map.
#[derive(Event)]
pub struct StartMapGeneration {
    /// The seed used to generate the map.
    pub seed: u32,

    /// The radius of the map.
    pub radius: u16,
}

/// An event send when the map is generated.
#[derive(Event)]
pub struct EndMapGeneration;

/// Generate each tiles of the map if the [StartMapGeneration] is received.
///
/// The map is generated using a [Perlin] noise and a [HexSpiral].
///
/// It's generated one tile at a time, until the spiral is finished.
fn generate_map(
    mut start_generation_events: EventReader<StartMapGeneration>,
    mut end_generation_writer: EventWriter<EndMapGeneration>,
    mut commands: Commands,
    mut local_noise: Local<Option<Perlin>>,
    mut local_spiral: Local<Option<HexSpiral<i32>>>,
) {
    // Handle map generation events and create the spiral and the noise.
    for event in start_generation_events.read() {
        *local_noise = Some(Perlin::new(event.seed));
        *local_spiral = Some(TilePosition::new(0, 0).spiral(event.radius as usize));
    }

    // Check if the map is being generated.
    let (Some(noise), Some(spiral)) = (local_noise.as_ref(), local_spiral.as_mut()) else {
        return;
    };

    // Spawn a tile until the spiral is finished
    // If the map is generated, we send [EndMapGeneration] and set the local
    // variables to None.
    if let Some(position) = spiral.next() {
        commands.spawn((get_tile_type(position, noise), position as TilePosition));
    } else {
        end_generation_writer.send(EndMapGeneration);
        *local_noise = None;
        *local_spiral = None;
    }
}

/// Returns the type of the [HexPosition] with the given noise.
fn get_tile_type(position: HexPosition<i32>, noise: &Perlin) -> Tile {
    let pixel_position = position.to_pixel_coordinates() / MAP_GENERATION_SCALE;
    let value = noise.get([pixel_position.x as f64, pixel_position.y as f64]);
    match value {
        v if v <= -0.4 => Tile::Hill,
        v if v >= 0.4 => Tile::Forest,
        _ => Tile::Grass,
    }
}

/// Despawns the tiles if the event [StartMapGeneration] is received.
fn delete_map(
    mut commands: Commands,
    query: Query<Entity, With<Tile>>,
    mut start_generation_events: EventReader<StartMapGeneration>,
) {
    for _ in start_generation_events.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
