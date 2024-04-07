//! All program related to the resources of the game.

use bevnet::{NetworkAppExt, Receive, SendTo};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Player;

/// The plugin that manage the resources.
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetResources>()
            .insert_resource(Resources::initial())
            .add_network_event::<UpdateResources>()
            .add_systems(
                Update,
                (handle_reset_resources, save_resources, update_resources),
            );
    }
}

/// The resources of the game.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub const fn initial() -> Self {
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

/// An event send to update the resources of a player.
#[derive(Event, Serialize, Deserialize, Clone, Copy)]
pub struct UpdateResources(pub Resources);

/// Save the resources of the game when you modify it.
fn save_resources(
    mut update_resources_event: EventWriter<SendTo<UpdateResources>>,
    resources: Res<Resources>,
    players: Query<&Player>,
) {
    if !resources.is_changed() {
        return;
    }

    let event = UpdateResources(*resources);

    for player in players.iter() {
        update_resources_event.send(SendTo(player.uuid, event));
    }
}

/// Update the resources of all player.
fn update_resources(
    mut update_resources_event: EventReader<Receive<UpdateResources>>,
    mut players: Query<&mut Player>,
) {
    for event in update_resources_event.read() {
        let Some(mut player) = players.iter_mut().find(|player| player.uuid == event.0) else {
            continue;
        };

        player.resources = event.1.0;
        println!(
            "Update resources for player {:?} to {:?}",
            player.uuid, player.resources
        );
    }
}
