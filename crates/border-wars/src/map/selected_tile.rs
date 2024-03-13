//! All programs related to the selection of a tile.

use bevy::prelude::*;

use super::renderer::TilesGap;
use super::Tile;

/// The event that is triggered when a tile is clicked.
///
/// The event contains the index (ID) of the clicked tile.
#[derive(Event)]
pub struct TileJustClicked(pub u32);

/// An event that is triggered when a mouse button is clicked.
///
/// The event contains the position of the cursor in the world.
#[derive(Event)]
struct ClickOnTheWorld(Vec2);

/// A zone that can't be clicked.
/// For exemple the UI of the game.
#[derive(Component)]
pub struct ZoneNotClickable;

/// The currently selected tile.
#[derive(Resource, Default, Debug)]
pub enum SelectedTile {
    /// The index (ID) of the selected tile.
    Tile(u32),

    /// Zero tile selected.
    #[default]
    None,
}

impl SelectedTile {
    /// Returns the index (ID) of the selected tile.
    /// Returns `None` if no tile is selected.
    pub const fn index(&self) -> Option<u32> {
        match self {
            Self::Tile(index) => Some(*index),
            Self::None => None,
        }
    }
}

/// A plugin that handles the selection of tiles.
pub struct SelectTilePlugin;

impl Plugin for SelectTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, mouse_handler)
            .add_systems(PreUpdate, select_closest_tile)
            .add_event::<ClickOnTheWorld>()
            .add_event::<TileJustClicked>()
            .init_resource::<SelectedTile>();
    }
}

/// Handles the mouse click and gets the position of the cursor in the world.
/// Finally, it sends an event with the position of the cursor.
fn mouse_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut events_writer: EventWriter<ClickOnTheWorld>,
    not_clickable_zones: Query<(&Node, &GlobalTransform), With<ZoneNotClickable>>,
    ui_scale: Res<UiScale>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.get_single().expect("Main window not found");

    let cursor_position_on_screen = window.cursor_position();

    let Some(cursor_position_on_screen) = cursor_position_on_screen else {
        return;
    };

    for (node, global_transform) in not_clickable_zones.iter() {
        let rect = node.physical_rect(global_transform, window.scale_factor(), ui_scale.0);
        if rect.contains(cursor_position_on_screen) {
            return;
        }
    }

    let (camera, camera_transform) = cameras.get_single().expect("Camera not found");

    let cursor_position_in_world = camera
        .viewport_to_world(camera_transform, cursor_position_on_screen)
        .expect("Failed to convert cursor position")
        .origin
        .truncate();

    events_writer.send(ClickOnTheWorld(cursor_position_in_world));
}

/// Get the closest tile to the cursor, send it in an event and select it.
fn select_closest_tile(
    tiles: Query<(Entity, &Transform, &Tile)>,
    mut click_event_reader: EventReader<ClickOnTheWorld>,
    mut clicked_tile_event_writer: EventWriter<TileJustClicked>,
    tile_gap: Res<TilesGap>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    for click_event in click_event_reader.read() {
        // The closest tile and its distance to the cursor.
        let mut closest_entity: Option<Entity> = None;
        let mut closest_position: Option<f32> = None;

        // To keep the aspect ratio.
        let click_position = click_event.0 / tile_gap.0;

        for (tile_entity, tile_transform, tile_type) in tiles.iter() {
            let tile_size = tile_type.get_image_size();
            let tile_scale = tile_transform.scale.truncate();

            let mut tile_position = tile_transform.translation.truncate() / tile_gap.0;
            // The origine of the tile is the bottom center.
            tile_position.y += (tile_size.y / 2.0) * tile_scale.y / tile_gap.0.y;

            let distance_to_cursor = tile_position.distance(click_position);

            if closest_position.is_none() || closest_position > Some(distance_to_cursor) {
                closest_entity = Some(tile_entity);
                closest_position = Some(distance_to_cursor);
            }
        }
        if let Some(tile_entity) = closest_entity {
            let entity_index = tile_entity.index();
            clicked_tile_event_writer.send(TileJustClicked(entity_index));
            if selected_tile.index() == Some(entity_index) {
                *selected_tile = SelectedTile::None;
            } else {
                *selected_tile = SelectedTile::Tile(entity_index);
            }
        }
    }
}
