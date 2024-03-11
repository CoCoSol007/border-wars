//! TODO

use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::render::color::Color;
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::{Node, Style, UiRect, Val};

pub mod tiles_info;

#[derive(Component)]
pub struct MainNode;

fn create_main_uui_node(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(bevy::ui::Val::Auto),
                width: Val::Px(1280.),
                height: Val::Px(720.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainNode);
}
