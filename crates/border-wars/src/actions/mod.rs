//! All programs related to the actions that you can do with all tiles.

use crate::map::Tile;

/// A number of action points. This is decreased when you do an action.
pub struct ActionPoint(pub u32);

/// The action that you can do with a tile.
pub enum Action {
    /// Build something on the tile.
    Build(u32),

    /// Destroy something on the tile.
    Destroy(u32),

    /// Recolt something on the tile.
    Recolt(u32),

    /// Upgrade the tile.
    Upgrade(u32),

    /// Manage the villages.
    VillageManagement,

    /// Manage the troops.
    TroopManagement,
}

impl Tile {
    /// Get the actions that you can do with the tile.
    pub fn get_action(&self, index_of_tile: u32) -> Vec<Action> {
        let mut actions = vec![];

        if let Self::Breeding
        | Self::Casern
        | Self::Castle
        | Self::Mine
        | Self::Outpost
        | Self::Sawmill
        | Self::Tower = *self
        {
            actions.push(Action::Upgrade(index_of_tile));
        }

        if let Self::Breeding | Self::Casern | Self::Mine | Self::Outpost | Self::Sawmill = *self {
            actions.push(Action::Destroy(index_of_tile));
        }

        if let Self::Breeding | Self::Casern = *self {
            actions.push(Action::TroopManagement);
        }

        if let Self::Breeding | Self::Hill = *self {
            actions.push(Action::Recolt(index_of_tile));
        }

        actions
    }
}

impl Action {
    /// Get the cost of an action.
    pub fn point_cost(&self) -> ActionPoint {
        ActionPoint(match *self {
            Action::Build(_) => 50,
            Action::Destroy(_) => 10,
            Action::Recolt(_) => 10,
            Action::Upgrade(_) => 25,
            Action::VillageManagement => 0,
            Action::TroopManagement => 0,
        })
    }
}
