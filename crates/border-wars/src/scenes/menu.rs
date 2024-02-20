//! The main menu of the game.

use bevy::prelude::*;

use crate::{change_scaling, CurrentScene};

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, menu_ui.run_if(in_state(CurrentScene::Menu)));
        app.add_systems(Update, change_scaling);
        app.add_systems(Update, handle_button);
    }
}

/// TODO
#[derive(Component, Clone)]
struct HoveredTexture {
    texture: Handle<Image>,
    hovered_texture: Handle<Image>,
}

/// Display the UI of the menu to host a game or join one.
fn menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                width: Val::Px(1280.),
                height: Val::Px(720.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            z_index: ZIndex::Local(0),
            background_color: Color::GREEN.into(),
            ..default()
        })
        .with_children(|node| main_node(node));

    create_side_button(
        UiRect {
            left: Val::Px(25.),
            right: Val::Auto,
            top: Val::Px(25.),
            bottom: Val::Auto,
        },
        CurrentScene::Lobby,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("button_settings_icon.png"),
            hovered_texture: asset_server.load("button_menu_icon.png"),
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
            texture: asset_server.load("button_menu_icon.png"),
            hovered_texture: asset_server.load("button_settings_icon.png"),
        },
    );
}

/// TODO
fn create_side_button(
    margin: UiRect,
    target_scene: CurrentScene,
    commands: &mut Commands,
    textures: HoveredTexture,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(75.),
                aspect_ratio: Some(1.),
                margin,
                ..default()
            },
            z_index: ZIndex::Local(1),
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert((target_scene, textures.clone()))
        .with_children(|button| {
            button.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                image: textures.texture.into(),
                ..default()
            });
        });
}

fn handle_button(
    interaction_query: Query<
        (&Interaction, &CurrentScene, &mut Children, &HoveredTexture),
        (Changed<Interaction>, With<Button>),
    >,
    mut image_query: Query<&mut UiImage>,
    mut next_scene: ResMut<NextState<CurrentScene>>,
) {
    for (interaction, target_scene, children, textures) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                next_scene.set(*target_scene);
            }
            Interaction::Hovered => {
                hover_system(children, &mut image_query, textures.texture.clone())
            }
            Interaction::None => hover_system(children, &mut image_query, textures.hovered_texture.clone()),
        }
    }
}

fn hover_system(
    children: &Children,
    image_query: &mut Query<&mut UiImage>,
    texture: Handle<Image>,
) {
    let mut image = image_query.get_mut(children[0]).unwrap();
    image.texture = texture
}

/// TODO
fn default_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.),
        height: Val::Percent(45.),
        margin: UiRect::all(Val::Auto),
        ..default()
    }
}

/// TODO
fn main_node(main_node: &mut ChildBuilder<'_, '_, '_>) {
    main_node.spawn(NodeBundle {
        style: Style {
            margin: UiRect {
                left: (Val::Auto),
                right: (Val::Auto),
                top: (Val::Px(25.)),
                bottom: (Val::Px(25.)),
            },
            width: Val::Px(650.),
            height: Val::Px(300.),
            ..default()
        },
        background_color: BackgroundColor(Color::RED),
        ..default()
    });

    main_node
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    left: (Val::Auto),
                    right: (Val::Auto),
                    top: (Val::Auto),
                    bottom: (Val::Px(25.)),
                },
                width: Val::Px(552.5),
                height: Val::Percent(70.),
                ..default()
            },
            ..default()
        })
        .with_children(|container| {
            container
                .spawn(NodeBundle {
                    style: default_style(),
                    background_color: Color::YELLOW_GREEN.into(),
                    ..default()
                })
                .with_children(|host| {
                    host.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    });
                    host.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    });
                });

            container
                .spawn(NodeBundle {
                    style: default_style(),
                    background_color: Color::YELLOW_GREEN.into(),
                    ..default()
                })
                .with_children(|join| {
                    join.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    });
                    join.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    });
                });
        });
}
