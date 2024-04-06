//! All program related to the resources of the game.

use bevy::prelude::*;

/// The plugin that manage the resources.
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetResources>()
            .insert_resource(Resources::initial())
            .add_systems(Update, handle_reset_resources);
    }
}

/// The resources of the game.
#[derive(Resource, Default)]
pub struct Resources {
    /// The stone resource.
    pub stone: u32,

    /// The wood resource.
    pub wood: u32,

    /// The food resource.
    pub food: u32,
}

impl Resources {
    /// Returns the initial resources of the game.
    const fn initial() -> Self {
        Self {
            stone: 100,
            wood: 100,
            food: 100,
        }
    }
}

/// An event send to reset the resources of the game.
#[derive(Event)]
pub struct ResetResources;

/// Handles the reset resources event.
fn handle_reset_resources(
    mut reset_resources_event: EventReader<ResetResources>,
    mut resources: ResMut<Resources>,
) {
    for _ in reset_resources_event.read() {
        *resources = Resources::initial();
    }
}
