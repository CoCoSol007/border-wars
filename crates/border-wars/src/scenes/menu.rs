//! The main menu of the game.

use bevnet::{Connection, SendTo, Uuid};
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_egui::egui::epaint::textures;
use bevy_egui::{egui, EguiContexts};

use crate::networking::connection::RequestJoin;
use crate::networking::PlayerRank;
use crate::ui::hover::HoveredTexture;
use crate::{CurrentScene, Player};

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, menu_ui.run_if(in_state(CurrentScene::Menu)));
        app.add_systems(OnEnter(CurrentScene::Menu), menu_ui2);
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
            });
        }
    });
}

/// A Component to identify menus entities.
/// In order to be able to remove them later.
#[derive(Component)]
struct MenuEntity;

type TargetScene = CurrentScene;

fn menu_ui2(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                width: Val::Px(1280.),
                height: Val::Px(720.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            image: asset_server.load("menu/bw_menu_bg.png").into(),
            z_index: ZIndex::Local(0),
            ..default()
        })
        .insert(MenuEntity);

    create_side_button(
        UiRect {
            left: Val::Px(25.),
            right: Val::Auto,
            top: Val::Px(25.),
            bottom: Val::Auto,
        },
        TargetScene::Lobby,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("menu/setting.png"),
            hovered_texture: asset_server.load("menu/setting_hover.png"),
        },
    );

    create_side_button(
        UiRect {
            left: Val::Auto,
            right: Val::Px(25.),
            top: Val::Px(25.),
            bottom: Val::Auto,
        },
        CurrentScene::Lobby,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("menu/info.png"),
            hovered_texture: asset_server.load("menu/info_hover.png"),
        },
    );

    renderer(
        &asset_server,
        commands,
        Some(HoveredTexture {
            texture: asset_server.load("menu/border_wars.png"),
            hovered_texture: asset_server.load("menu/border_wars.png"),
        }),
        None,
        Val::Px(78.),
        Val::Px(614.),
        Val::Px(25.),
        Val::Px(333.),
    )
}

/// A function to create a side button.
fn create_side_button(
    margin: UiRect,
    target_scene: TargetScene,
    commands: &mut Commands,
    textures: HoveredTexture,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(53.),
                aspect_ratio: Some(1.),
                margin,
                ..default()
            },
            z_index: ZIndex::Global(14),
            image: textures.texture.clone().into(),
            ..default()
        })
        .insert((target_scene, textures, MenuEntity));
}

fn renderer(
    asset_server: &Res<AssetServer>,
    mut background: Commands,
    textures: Option<HoveredTexture>,
    texture: Option<&str>,
    height: Val,
    width: Val,
    top: Val,
    left: Val,
) {
    let mut i = background
        .spawn(ImageBundle {
            style: Style {
                height,
                width,
                top,
                left,
                ..default()
            },
            image: match textures {
                Some(ref textures) => textures.texture.clone().into(),
                _ => match texture {
                    Some(texture) => asset_server.load(texture).into(),
                    _ => panic!("PAS DE TEXTURES FRER"),
                },
            },
            ..default()
        });
        if textures.is_some() {
            i.insert(textures.unwrap());
        }
        
}
