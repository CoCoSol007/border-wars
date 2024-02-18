//! The main menu of the game.

use bevy::prelude::*;

use crate::{change_scaling, CurrentScene};

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, menu_ui.run_if(in_state(CurrentScene::Menu)));
        app.add_systems(Update, change_scaling);
    }
}

/// Display the UI of the menu to host a game or join one.
fn menu_ui(mut commands: Commands) {
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
            ..default()
        })
        .with_children(|main_node| {
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
        });

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(75.),
            aspect_ratio: Some(1.),
            margin: UiRect {
                left: Val::Px(25.),
                right: Val::Auto,
                top: Val::Px(25.),
                bottom: Val::Auto,
            },
            ..default()
        },
        z_index: ZIndex::Local(1),
        background_color: BackgroundColor(Color::BLUE),
        ..default()
    });

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(75.),
            aspect_ratio: Some(1.),
            margin: UiRect {
                left: Val::Auto,
                right: Val::Px(25.),
                top: Val::Px(25.),
                bottom: Val::Auto,
            },
            ..default()
        },
        z_index: ZIndex::Local(1),
        background_color: BackgroundColor(Color::BLUE),
        ..default()
    });
}

fn default_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.),
        height: Val::Percent(45.),
        margin: UiRect::all(Val::Auto),
        ..default()
    }
}
