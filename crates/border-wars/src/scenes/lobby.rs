//! The lobby of the game.

use bevnet::{Connection, SendTo};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand::Rng;

use crate::map::generation::StartMapGeneration;
use crate::networking::connection::RemovePlayer;
use crate::networking::{PlayerRank, StartGame};
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
    connection: Res<Connection>,
    all_players_query: Query<&Player>,
    mut kick_player: EventWriter<SendTo<RemovePlayer>>,
    mut map_size: Local<u32>,
    mut start_game_event: EventWriter<SendTo<StartGame>>,
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

        if self_player.rank != PlayerRank::Admin {
            return;
        }

        ui.add(egui::Slider::new(&mut (*map_size), 1..=5).text("map size"));

        if !ui.button("Run the game").clicked() {
            return;
        }

        let seed = rand::thread_rng().gen::<u32>();
        let index = *map_size as usize;
        let nomber_of_players = all_players_query.iter().count() as u32;

        let radius = get_map_sizes(nomber_of_players)[index] as u16 * 2;

        // Start the game.
        for player in all_players_query.iter() {
            start_game_event.send(SendTo(
                player.uuid,
                StartGame(StartMapGeneration { seed, radius }),
            ));
        }
    });
}

/// Get the map sizes form a given number of players.
fn get_map_sizes(number_of_players: u32) -> Vec<u32> {
    let mut result = Vec::with_capacity(6);

    let mut current = 0;
    while result.len() < 6 {
        current += 1;

        if (current * 6) % number_of_players == 0 {
            result.push(current);
        }
    }

    result
}
