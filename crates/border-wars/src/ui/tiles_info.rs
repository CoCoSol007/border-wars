//! TODO

use bevy::prelude::*;

use super::{create_main_uui_node, MainNode};
use crate::map::click_tile::TileJustClicked;
use crate::map::Tile;

/// TODO
pub struct TilesInfoPlugin;

impl Plugin for TilesInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_tile_click)
            .init_resource::<SelectedTile>()
            .add_systems(Update, update_tile_info_text)
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

#[derive(Resource, Default)]
enum SelectedTile {
    Tile(Tile, u32),
    #[default]
    None,
}

impl SelectedTile {
    fn index(&self) -> Option<u32> {
        match self {
            SelectedTile::Tile(_, index) => Some(*index),
            SelectedTile::None => None,
        }
    }
}

fn handle_tile_click(
    mut event: EventReader<TileJustClicked>,
    mut query: Query<(&Tile, Entity, &mut Transform)>,
    mut selected: ResMut<SelectedTile>,
) {
    if let Some(event) = event.read().last() {
        let save_selected = selected.index();
        for (_, entity, mut transform) in query.iter_mut() {
            if selected.index() == Some(entity.index()) {
                if event.0 == entity.index() {
                    *selected = SelectedTile::None;
                }
                transform.translation.y -= 10.;
            }
        }

        for (tile, entity, mut transform) in query.iter_mut() {
            if event.0 == entity.index() && save_selected != Some(event.0) {
                *selected = SelectedTile::Tile(*tile, entity.index());
                transform.translation.y += 10.;
            }
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
            }).insert(TileInfoBox)
            .with_children(|builder| {
                builder
                    .spawn(TextBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
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
    mut query: Query<(&mut Transform, &mut Text), With<TileInfoText>>,
    selected: Res<SelectedTile>,
    mut main_nodes: Query<&mut Transform, With<TileInfoBox>>,
) {
    let mut info_box = main_nodes.single_mut();
    for (mut transform, mut text) in query.iter_mut() {
        if selected.index().is_none() {
            info_box.translation.y = -100.0;
            text.sections = vec![];
            return;
        }
        if let SelectedTile::Tile(tile, _) = *selected {
            text.sections = vec![TextSection {
                value: tile.get_info_text(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            }];
            transform.translation.z = 1.0;
        }
    }
}