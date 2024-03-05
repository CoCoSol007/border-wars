//! Contains all the logic related to the map.

pub mod generation;
pub mod hex;
pub mod renderer;

use bevy::prelude::*;

use self::hex::*;

/// The position of a tile in a hexagonal map.
pub type TilePosition = HexPosition<i32>;

/// The tile of the map.
#[derive(Component, Debug)]
pub enum Tile {
    /// The hill tile.
    Hill,

    /// The grass tile.
    Grass,

    /// The forest tile.
    Forest,
}
