//! ToDo

use bevy::prelude::*;

use crate::CurrentScene;

/// The plugin that sets up the UI for the game.
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CurrentScene::Game), setup_ui);
    }
}

/// Sets up the UI for the game.
fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Px(25.),
            },
            width: Val::Px(1000.),
            height: Val::Px(150.),
            ..Default::default()
        },
        background_color: Color::BLUE.into(),
        ..Default::default()
    });

    commands.spawn(NodeBundle {
        style: Style {
            margin: UiRect {
                left: Val::Px(10.),
                right: Val::Auto,
                top: Val::Px(10.),
                bottom: Val::Auto,
            },
            width: Val::Px(200.),
            height: Val::Px(200.),
            ..Default::default()
        },
        background_color: Color::BLUE.into(),
        ..Default::default()
    });
}
