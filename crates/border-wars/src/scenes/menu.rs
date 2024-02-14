//! The main menu of the game.

use bevy::prelude::*;

use crate::CurrentScene;

/// The plugin for the menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, menu_ui.run_if(in_state(CurrentScene::Menu)));
    }
}
/// Display the UI of the menu to host a game or join one.
fn menu_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Green
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: (Val::Auto),
                    right: (Val::Auto),
                    top: (Val::Px(0.)),
                    bottom: (Val::Px(0.)),
                },
                width: Val::Percent(55.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        // Text
        .with_children(|parent| {
            parent.spawn(
                NodeBundle{
                    style: Style {
                        margin: UiRect {
                            left: (Val::Auto),
                            right: (Val::Auto),
                            top: (Val::Percent(5.)),
                            bottom: (Val::Percent(15.)),
                        },
                        width: Val::Percent(90.),
                        height: Val::Percent(25.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::RED),
                    ..default()
                }
                
            );
        })
        // BLUE
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: (Val::Auto),
                            right: (Val::Auto),
                            top: (Val::Auto),
                            bottom: (Val::Percent(5.)),
                        },
                        width: Val::Percent(85.),
                        height: Val::Percent(70.),
                        ..default()
                    },
                    ..default()
                })
                // YELLOW_GREEN 1
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(45.),
                            margin: UiRect {
                                left: (Val::Auto),
                                right: (Val::Auto),
                                top: (Val::Auto),
                                bottom: (Val::Auto),
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(Color::YELLOW_GREEN),
                        ..default()
                    });
                })
                // YELLOW_GREEN 2
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(45.),
                            margin: UiRect {
                                left: (Val::Auto),
                                right: (Val::Auto),
                                top: (Val::Auto),
                                bottom: (Val::Auto),
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(Color::YELLOW_GREEN),
                        ..default()
                    });
                });
        });
}
