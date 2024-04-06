//! The main menu of the game.

use bevnet::{Connection, SendTo, Uuid};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::networking::connection::RequestJoin;
use crate::networking::PlayerRank;
use crate::{CurrentScene, Player};

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
    mut request_join: EventWriter<SendTo<RequestJoin>>,
    mut name: Local<String>,
    connection: Res<Connection>,
    mut commands: Commands,
) {
    let Some(uuid) = connection.identifier() else {
        return;
    };

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.heading("Border Wars");

        ui.separator();

        ui.label("Name");
        ui.text_edit_singleline(&mut *name);

        ui.separator();

        ui.label("Connect to an existing game:");
        ui.horizontal(|ui| {
            ui.label("Game ID: ");
            ui.text_edit_singleline(&mut *connection_string);

            let Ok(game_id) = Uuid::parse_str(&connection_string) else {
                return;
            };

            if ui.button("Join").clicked() {
                next_scene.set(CurrentScene::Lobby);
                request_join.send(SendTo(
                    game_id,
                    RequestJoin(Player {
                        name: name.clone(),
                        rank: PlayerRank::Player,
                        uuid,
                        color: rand::random::<(u8, u8, u8)>(),
                    }),
                ));
            }
        });

        ui.separator();

        if ui.button("Create new game").clicked() {
            next_scene.set(CurrentScene::Lobby);
            commands.spawn(Player {
                name: name.clone(),
                rank: PlayerRank::Admin,
                uuid,
                color: rand::random::<(u8, u8, u8)>(),
            });
        }
    });
}
