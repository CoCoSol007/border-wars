//! All programs related to the actions that you can do with all tiles.

use bevy::prelude::*;

use crate::map::Tile;

/// A number of action points. This is decreased when you do an action.
pub struct ActionPoint(pub u32);

/// The action that you can do with a tile.
pub enum Action {
    /// Build something on the tile.
    Build(Entity),

    /// Destroy something on the tile.
    Destroy(Entity),

    /// Recolt something on the tile.
    Recolt(Entity),

    /// Upgrade the tile.
    Upgrade(Entity),

    /// Manage the villages.
    VillageManagement,

    /// Manage the troops.
    TroopManagement,

    /// Teleport management.
    TeleportManagement(Entity),
}

impl Tile {
    /// Get the actions that you can do with the tile.
    pub fn get_action(&self, tile_entity: Entity) -> Vec<Action> {
        let mut actions = vec![];

        if let Self::Breeding
        | Self::Casern
        | Self::Castle
        | Self::Mine
        | Self::Sawmill
        | Self::Tower = *self
        {
            actions.push(Action::Upgrade(tile_entity));
        }

        if let Self::Breeding | Self::Casern | Self::Mine | Self::Outpost | Self::Sawmill = *self {
            actions.push(Action::Destroy(tile_entity));
        }

        if matches!(*self, Self::Casern) {
            actions.push(Action::TroopManagement);
        }

        if let Self::Forest | Self::Hill = *self {
            actions.push(Action::Recolt(tile_entity));
        }

        if matches!(*self, Self::Castle) {
            actions.push(Action::VillageManagement);
        }

        if matches!(*self, Self::Grass) {
            actions.push(Action::Build(tile_entity));
        }

        if matches!(*self, Self::Outpost) {
            actions.push(Action::TeleportManagement(tile_entity));
        }

        actions
    }
}

impl Action {
    /// Get the cost of an action.
    pub const fn point_cost(&self) -> ActionPoint {
        ActionPoint(match *self {
            Self::Build(_) => 50,
            Self::Destroy(_) => 10,
            Self::Recolt(_) => 10,
            Self::Upgrade(_) => 25,
            Self::VillageManagement => 0,
            Self::TroopManagement => 0,
            Self::TeleportManagement(_) => 0,
        })
    }
}
