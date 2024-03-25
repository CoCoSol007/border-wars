//! The lobby of the game.

use bevnet::{Connection, SendTo};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::networking::connection::RemovePlayer;
use crate::networking::PlayerRank;
use crate::{CurrentScene, Player};

/// The plugin for the lobby.
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lobby_ui.run_if(in_state(CurrentScene::Lobby)));
    }
}

/// Display the UI of the lobby.
fn lobby_ui(
    mut ctx: EguiContexts,
    mut next_scene: ResMut<NextState<CurrentScene>>,
    connection: Res<Connection>,
    all_players_query: Query<&Player>,
    mut kick_player: EventWriter<SendTo<RemovePlayer>>,
) {
    // Get our player info.
    let Some(self_player) = all_players_query
        .iter()
        .find(|player| connection.identifier() == Some(player.uuid))
    else {
        return;
    };

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.heading("Border Wars");

        ui.separator();

        ui.label("Game created");
        ui.horizontal(|ui| {
            if self_player.rank != PlayerRank::Admin {
                return;
            }
            ui.label("Game ID: ");
            // TODO : get the game ID and display it.
            ui.text_edit_singleline(&mut connection.identifier().unwrap_or_default().to_string());
        });

        ui.separator();

        for player in all_players_query.iter() {
            ui.label(player.name.to_string());
            if self_player.rank == PlayerRank::Admin
                && player.rank != PlayerRank::Admin
                && ui.button("Remove").clicked()
            {
                for sender_id in all_players_query.iter() {
                    kick_player.send(SendTo(sender_id.uuid, RemovePlayer(player.clone())));
                }
            }
            ui.separator();
        }

        if self_player.rank == PlayerRank::Admin && ui.button("Run the game").clicked() {
            next_scene.set(CurrentScene::Game);
            // TODO: run the game
        }
    });
}
