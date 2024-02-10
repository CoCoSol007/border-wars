//! The main menu of the game.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::CurrentScene;

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu_ui.run_if(in_state(CurrentScene::Menu)));
    }
}
/// Display the UI of the menu to host a game or join one.
fn menu_ui(
    mut ctx: EguiContexts,
    mut connection_string: Local<String>,
    mut next_scene: ResMut<NextState<CurrentScene>>,
) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.heading("Border Wars");

        ui.separator();

        ui.label("Connect to an existing game:");
        ui.horizontal(|ui| {
            ui.label("Game ID: ");
            ui.text_edit_singleline(&mut *connection_string);

            if ui.button("Join").clicked() {
                next_scene.set(CurrentScene::Game);
                // TODO: connect to the game
            }
        });

        ui.separator();

        if ui.button("Create new game").clicked() {
            next_scene.set(CurrentScene::Lobby);
            // TODO: create a new game
        }
    });
}
