//! The lobby of the game.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::CurrentScene;

/// The plugin for the lobby.
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lobby_ui.run_if(in_state(CurrentScene::Lobby)));
    }
}

/// Display the UI of the lobby.
fn lobby_ui(mut ctx: EguiContexts, mut next_scene: ResMut<NextState<CurrentScene>>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.heading("Border Wars");

        ui.separator();

        ui.label("Game created");
        ui.horizontal(|ui| {
            ui.label("Game ID: ");
            // TODO : get the game ID and display it.
            ui.label("connection_string");
        });

        ui.separator();

        if ui.button("Run the game").clicked() {
            next_scene.set(CurrentScene::Game);
            // TODO: run the game
        }
    });
}
