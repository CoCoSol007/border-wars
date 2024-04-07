//! Contains all the logic related to the map.

pub mod generation;
pub mod hex;
pub mod ownership;
pub mod renderer;
pub mod selected_tile;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use self::hex::*;

/// The position of a tile in a hexagonal map.
pub type TilePosition = HexPosition<i32>;

/// The tile of the map.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tile {
    /// The breeding tile.
    Breeding,

    /// The Casern tile.
    Casern,

    /// The castle tile.
    Castle,

    /// The hill tile.
    Hill,

    /// The grass tile.
    Grass,

    /// The forest tile.
    Forest,

    /// The mine tile.
    Mine,

    /// The outpost tile
    Outpost,

    /// The sawmill tile
    Sawmill,

    /// The tower tile
    Tower,

    /// The wall tile
    Wall,
}

impl Tile {
    /// Returns the text representation of the tile.
    pub fn to_text(&self) -> String {
        match self {
            Self::Breeding => "breeding".to_string(),
            Self::Casern => "casern".to_string(),
            Self::Castle => "castle".to_string(),
            Self::Forest => "forest".to_string(),
            Self::Grass => "grass".to_string(),
            Self::Hill => "hill".to_string(),
            Self::Mine => "mine".to_string(),
            Self::Outpost => "outpost".to_string(),
            Self::Sawmill => "sawmill".to_string(),
            Self::Tower => "tower".to_string(),
            Self::Wall => "wall".to_string(),
        }
    }
}
