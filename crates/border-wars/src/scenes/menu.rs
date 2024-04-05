//! The main menu of the game.

use bevnet::{Connection, SendTo, Uuid};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::networking::connection::RequestJoin;
use crate::networking::PlayerRank;
use crate::ui::hover::HoveredTexture;
use crate::{CurrentScene, Player, Scene};

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

/// A Component to identify menus entities.
/// In order to be able to remove them later.
#[derive(Component)]
struct MenuEntity;

type TargetScene = crate::Scene;

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
        .insert(MenuEntity)
        .with_children(|child: &mut ChildBuilder| renderer(child, &asset_server));

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
        TargetScene::Lobby,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("menu/info.png"),
            hovered_texture: asset_server.load("menu/info_hover.png"),
        },
    );
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

fn renderer(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/border_wars.png"),
            ..default()
        },),
        Val::Px(78.),
        Val::Px(614.),
        Val::Px(25.),
        Val::Px(333.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/host_icon.png"),
            ..default()
        },),
        Val::Px(42.),
        Val::Px(53.),
        Val::Px(223.),
        Val::Px(356.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/host.png"),
            ..default()
        },),
        Val::Px(38.),
        Val::Px(105.),
        Val::Px(229.),
        Val::Px(429.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/trait.png"),
            ..default()
        },),
        Val::Px(7.),
        Val::Px(427.),
        Val::Px(279.),
        Val::Px(426.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/button.png"),
            ..default()
        },),
        Val::Px(34.),
        Val::Px(253.),
        Val::Px(299.),
        Val::Px(513.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/join_icon.png"),
            ..default()
        },),
        Val::Px(41.),
        Val::Px(63.),
        Val::Px(393.),
        Val::Px(353.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/join.png"),
            ..default()
        },),
        Val::Px(38.),
        Val::Px(101.),
        Val::Px(392.),
        Val::Px(428.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/trait.png"),
            ..default()
        },),
        Val::Px(7.),
        Val::Px(427.),
        Val::Px(443.),
        Val::Px(426.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/button.png"),
            ..default()
        },),
        Val::Px(34.),
        Val::Px(253.),
        Val::Px(463.),
        Val::Px(513.),
    );

    render(
        commands,
        (UiImage {
            texture: asset_server.load("menu/airplane.png"),
            ..default()
        },),
        Val::Px(30.),
        Val::Px(35.),
        Val::Px(465.),
        Val::Px(777.),
    );
}

fn render<T: Bundle>(
    background: &mut ChildBuilder,
    textures: T,
    height: Val,
    width: Val,
    top: Val,
    left: Val,
) {
    background
        .spawn(ImageBundle {
            style: Style {
                height,
                width,
                top,
                left,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(textures);
}
