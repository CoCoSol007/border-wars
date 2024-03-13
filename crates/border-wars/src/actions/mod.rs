//! All programs related to the actions that you can do with all tiles.

use crate::map::Tile;

pub enum Action {
    Build(u32),
    Destroy(u32),
    Recolt(u32),
    Upgrade(u32),
    VillageManagement,
    TroopManagement,
}

impl Tile {
    pub fn get_action(&self, index_of_tile: u8) -> Vec<Action> {
        match *self {
            Self::Breeding => [
                Action::Destroy(index_of_tile),
                Action::Upgrade(index_of_tile),
            ],
            Self::Casern => [
                Action::Destroy(index_of_tile),
                Action::Upgrade(index_of_tile),
                Action::TroopManagement,
            ],
            Self::Castle => [
                Action::Upgrade(index_of_tile),
                Action::VillageManagement,
            ],
            Self::Hill => [
                Action::Recolt(index_of_tile),
            ],
            Self::Grass => [
                Action::Build(index_of_tile),
            ],
            Self::Forest => [
                Action::Recolt(index_of_tile),
            ],
            Self::Mine => [
                Action::Destroy(index_of_tile),
                Action::Upgrade(index_of_tile),
            ],
            Self::Outpost => [
                Action::Destroy(index_of_tile),
            ],
            Self::Sawmill => [
                Action::Destroy(index_of_tile),
                Action::Recolt(index_of_tile),
            ],
            Self::Tower => [
                Action::Destroy(index_of_tile),
                Action::Upgrade(index_of_tile),
            ],
            Self::Wall => [],
        }
    }
}
