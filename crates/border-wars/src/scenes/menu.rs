//! The main menu of the game.

// use bevnet::{Connection, SendTo, Uuid};
use bevy::prelude::*;
// use bevy_egui::{egui, EguiContexts};

// use crate::networking::connection::RequestJoin;
// use crate::networking::PlayerRank;
use crate::ui::hover::HoveredTexture;
use crate::CurrentScene;
// use crate::Player;

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, menu_ui.run_if(in_state(CurrentScene::Menu)));
        app.add_systems(OnEnter(CurrentScene::Menu), menu_ui);
    }
}

#[derive(Component)]
struct Host;

#[derive(Component)]
struct Join;

// Display the UI of the menu to host a game or join one.
// fn old_menu(
//     mut ctx: EguiContexts,
//     mut connection_string: Local<String>,
//     mut next_scene: ResMut<NextState<CurrentScene>>,
//     mut request_join: EventWriter<SendTo<RequestJoin>>,
//     mut name: Local<String>,
//     connection: Res<Connection>,
//     mut commands: Commands,
// ) {
//     let Some(uuid) = connection.identifier() else {
//         return;
//     };

//     egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
//         ui.heading("Border Wars");

//         ui.separator();

//         ui.label("Name");
//         ui.text_edit_singleline(&mut *name);

//         ui.separator();

//         ui.label("Connect to an existing game:");
//         ui.horizontal(|ui| {
//             ui.label("Game ID: ");
//             ui.text_edit_singleline(&mut *connection_string);

//             let Ok(game_id) = Uuid::parse_str(&connection_string) else {
//                 return;
//             };

//             if ui.button("Join").clicked() {
//                 next_scene.set(CurrentScene::Lobby);
//                 request_join.send(SendTo(
//                     game_id,
//                     RequestJoin(Player {
//                         name: name.clone(),
//                         rank: PlayerRank::Player,
//                         uuid,
//                         color: rand::random::<(u8, u8, u8)>(),
//                     }),
//                 ));
//             }
//         });

//         ui.separator();

//         if ui.button("Create new game").clicked() {
//             next_scene.set(CurrentScene::Lobby);
//             commands.spawn(Player {
//                 name: name.clone(),
//                 rank: PlayerRank::Admin,
//                 uuid,
//                 color: rand::random::<(u8, u8, u8)>(),
//             });
//         }
//     });
// }

/// A Component to identify menus entities.
/// In order to be able to remove them later.
#[derive(Component)]
struct MenuEntity;

type TargetScene = crate::Scene;

/// Create main element for the menu
fn menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the background.
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                width: px(1280.),
                height: px(720.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            image: asset_server.load("menu/bw_menu_bg.png").into(),
            z_index: ZIndex::Local(0),
            ..default()
        })
        .insert(MenuEntity)
        .with_children(|child: &mut ChildBuilder| creation_menu_ui(child, &asset_server));

    // Create the settings button.
    create_side_button(
        UiRect {
            left: px(25.),
            right: Val::Auto,
            top: px(25.),
            bottom: Val::Auto,
        },
        TargetScene::Lobby,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("menu/setting.png"),
            hovered_texture: asset_server.load("menu/setting_hover.png"),
        },
    );

    // Create the info button.
    create_side_button(
        UiRect {
            left: Val::Auto,
            right: px(25.),
            top: px(25.),
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
                width: px(53.),
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

/// That function create all elements of the menu.
fn creation_menu_ui(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    // Create the title.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/border_wars.png"),
            ..default()
        },),
        (px(614.), px(78.)),
        (px(333.), px(25.)),
    );

    // Create the host icon.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/host_icon.png"),
            ..default()
        },),
        (px(53.), px(42.)),
        (px(356.), px(223.)),
    );

    // Create the host title.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/host.png"),
            ..default()
        },),
        (px(105.), px(38.)),
        (px(429.), px(229.)),
    );

    // Create the host line.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/line.png"),
            ..default()
        },),
        (px(427.), px(7.)),
        (px(426.), px(279.)),
    );

    // Create the host button.
    create_object(
        commands,
        (
            UiImage {
                texture: asset_server.load("menu/button.png"),
                ..default()
            },
            Button,
            Host,
        ),
        (px(253.), px(34.)),
        (px(513.), px(299.)),
    );

    // Create the join icon.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/join_icon.png"),
            ..default()
        },),
        (px(63.), px(41.)),
        (px(353.), px(393.)),
    );

    // Create the join title.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/join.png"),
            ..default()
        },),
        (px(101.), px(38.)),
        (px(428.), px(392.)),
    );

    // Create the join line.
    create_object(
        commands,
        (UiImage {
            texture: asset_server.load("menu/line.png"),
            ..default()
        },),
        (px(427.), px(7.)),
        (px(426.), px(443.)),
    );

    // Create the join button.
    create_object(
        commands,
        (
            UiImage {
                texture: asset_server.load("menu/button.png"),
                ..default()
            },
            Button,
            Join,
        ),
        (px(253.), px(34.)),
        (px(513.), px(463.)),
    );

    // Create the airplane.
    create_object(
        commands,
        (
            UiImage {
                texture: asset_server.load("menu/airplane.png"),
                ..default()
            },
            Button,
        ),
        (px(35.), px(30.)),
        (px(777.), px(465.)),
    );
}

/// A function that create objets with the given parameters.
fn create_object<T: Bundle>(
    background: &mut ChildBuilder,
    bundle: T,
    size: (Val, Val),
    pos: (Val, Val),
) {
    background
        .spawn(ImageBundle {
            style: Style {
                width: size.0,
                height: size.1,
                left: pos.0,
                top: pos.1,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(bundle);
}

/// Translate a f32 into a `Val::Px'.
fn px(value: f32) -> Val {
    Val::Px(value)
}
