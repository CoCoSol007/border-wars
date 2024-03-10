//! The main menu of the game.

use bevy::prelude::*;

use crate::{change_scaling, CurrentScene};

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CurrentScene::Menu), menu_ui);
        app.add_systems(Update, change_scaling);
        app.add_systems(Update, hover_system);
        app.add_systems(Update, pressed_system);
        app.add_systems(OnExit(CurrentScene::Menu), destroy_menu);
    }
}

/// TODO
#[derive(Component, Clone)]
struct HoveredTexture {
    /// TODO
    texture: Handle<Image>,

    /// TODO
    hovered_texture: Handle<Image>,
}

/// TODO
#[derive(Component)]
struct MenuEntity;

/// Display the UI of the menu to host a game or join one.
fn menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

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
            ..default()
        })
        .insert(MenuEntity)
        .with_children(|child: &mut ChildBuilder| main_node(child, &asset_server));

    create_side_button(
        UiRect {
            left: Val::Px(25.),
            right: Val::Auto,
            top: Val::Px(25.),
            bottom: Val::Auto,
        },
        CurrentScene::Setting,
        &mut commands,
        HoveredTexture {
            texture: asset_server.load("button_settings_icon.png"),
            hovered_texture: asset_server.load("button_settings_icon_hover.png"),
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
            hovered_texture: asset_server.load("button_menu_icon_hover.png"),
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
            z_index: ZIndex::Global(14),
            image: textures.texture.clone().into(),
            ..default()
        })
        .insert((target_scene, textures, MenuEntity));
}

/// TODO
fn create_button(
    target_scene: CurrentScene,
    commands: &mut ChildBuilder,
    textures: HoveredTexture,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(266.),
                height: Val::Px(70.),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            z_index: ZIndex::Global(1),
            image: textures.texture.clone().into(),
            ..default()
        })
        .insert((target_scene, textures, MenuEntity));
}

/// TODO
fn pressed_system(
    interaction_query: Query<(&Interaction, &CurrentScene), (Changed<Interaction>, With<Button>)>,
    mut next_scene: ResMut<NextState<CurrentScene>>,
) {
    for (interaction, target_scene) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_scene.set(*target_scene);
        }
    }
}

/// TODO
fn hover_system(
    mut interaction_query: Query<
        (&Interaction, &HoveredTexture, &mut UiImage),
        Changed<Interaction>,
    >,
) {
    for (interaction, textures, mut image) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => image.texture = textures.hovered_texture.clone(),
            Interaction::None => image.texture = textures.texture.clone(),
            Interaction::Pressed => (),
        }
    }
}

/// TODO
fn default_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.),
        height: Val::Percent(55.),
        margin: UiRect::all(Val::Auto),
        ..default()
    }
}

/// TODO
fn main_node(main_node: &mut ChildBuilder<'_, '_, '_>, asset_server: &Res<AssetServer>) {
    main_node
        .spawn(ImageBundle {
            style: Style {
                height: Val::Px(150.),
                width: Val::Px(613.5),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(25.),
                    bottom: Val::Px(50.),
                },
                ..default()
            },
            image: asset_server.load("bw.png").into(),
            ..default()
        })
        .insert(MenuEntity);

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
                width: Val::Px(500.),
                height: Val::Percent(65.),
                ..default()
            },
            ..default()
        })
        .with_children(|container| {
            container
                .spawn(NodeBundle {
                    style: default_style(),
                    ..default()
                })
                .insert(MenuEntity)
                .with_children(|host| {
                    host.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    })
                    .insert(MenuEntity);
                    create_button(
                        CurrentScene::Lobby,
                        host,
                        HoveredTexture {
                            texture: asset_server.load("host.png"),
                            hovered_texture: asset_server.load("host.png"),
                        },
                    )
                });

            container
                .spawn(NodeBundle {
                    style: default_style(),
                    ..default()
                })
                .insert(MenuEntity)
                .with_children(|join| {
                    join.spawn(NodeBundle {
                        style: default_style(),
                        background_color: BackgroundColor(Color::YELLOW),
                        ..default()
                    })
                    .insert(MenuEntity);
                    create_button(
                        CurrentScene::Game,
                        join,
                        HoveredTexture {
                            texture: asset_server.load("join.png"),
                            hovered_texture: asset_server.load("join.png"),
                        },
                    )
                });
        });
}

/// The function that destroys the menu.
fn destroy_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
