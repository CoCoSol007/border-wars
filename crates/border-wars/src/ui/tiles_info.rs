//! TODO

use bevy::prelude::*;

use super::{create_main_uui_node, MainNode};
use crate::map::selected_tile::SelectedTile;
use crate::map::Tile;

/// TODO
pub struct TilesInfoPlugin;

impl Plugin for TilesInfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .add_systems(
                Update,
                update_tile_info_text.run_if(in_state(crate::CurrentScene::Game)),
            )
            .add_systems(OnEnter(crate::CurrentScene::Game), init_text_zone)
            .add_systems(Startup, create_main_uui_node);
    }
}

impl Tile {
    fn get_info_text(&self) -> String {
        match self {
            Tile::Hill => "This is a hill".to_string(),
            Tile::Grass => "This is grass".to_string(),
            Tile::Forest => "This is a forest".to_string(),
        }
    }
}

fn init_text_zone(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_nodes: Query<Entity, With<MainNode>>,
) {
    let main_node = main_nodes.single();

    commands.entity(main_node).with_children(|parent| {
        parent
            .spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,

                    bottom: Val::ZERO,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },

                    width: Val::Percent(50.0),
                    height: Val::Px(100.),
                    ..Default::default()
                },

                image: UiImage {
                    texture: asset_server.load("temp.png"),
                    ..default()
                },
                ..default()
            })
            .insert(TileInfoBox)
            .with_children(|builder| {
                builder
                    .spawn(TextBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            ..default()
                        },
                        z_index: bevy::prelude::ZIndex::Global(10),
                        ..default()
                    })
                    .insert(TileInfoText);
            });
    });
}

/// TODO
#[derive(Component)]
pub struct TileInfoText;

/// TODO
#[derive(Component)]
pub struct TileInfoBox;

fn update_tile_info_text(
    mut textes_info: Query<&mut Text, With<TileInfoText>>,
    selected: Res<SelectedTile>,
    tiles: Query<&Tile>,
) {
    let mut text_info = textes_info.single_mut();
    if let SelectedTile::Tile(tile) = *selected {
        let Ok(tile) = tiles.get(tile) else {
            return;
        };
        text_info.sections = vec![TextSection {
            value: tile.get_info_text(),
            style: TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        }];
    }
}
