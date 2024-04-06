//! All functions related to the rendering of the map.

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::map::{Tile, TilePosition};

/// A plugin to render the map.
pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_resources_for_rendering)
            .add_systems(
                Update,
                render_map.run_if(in_state(crate::CurrentScene::Game)),
            )
            .insert_resource(ClearColor(Color::rgb_u8(129, 212, 250)));
    }
}

/// The gap between the center of the tiles in the map.
#[derive(Resource)]
pub struct TilesGap(pub Vec2);

/// The size of the tiles in the map.
#[derive(Resource, Clone, Copy)]
struct TilesSize(Vec2);

impl Tile {
    /// Returns the handle of the image of the tile.
    fn get_texture(&self, asset_server: &AssetServer) -> Handle<Image> {
        asset_server.load(format!("tiles/{}.png", self.to_text()))
    }

    /// Returns the size of the image of the tile.
    ///
    /// TODO: we are currently using temporary images that will modify
    /// this function in the future.
    pub const fn get_image_size(&self) -> Vec2 {
        match self {
            Self::Breeding => Vec2::new(184., 158.),
            Self::Casern => Vec2::new(184., 167.),
            Self::Castle => Vec2::new(192., 196.),
            Self::Forest => Vec2::new(184., 165.),
            Self::Grass => Vec2::new(184., 138.),
            Self::Hill => Vec2::new(184., 181.),
            Self::Mine => Vec2::new(184., 166.),
            Self::Outpost => Vec2::new(184., 208.),
            Self::Sawmill => Vec2::new(184., 138.),
            Self::Tower => Vec2::new(184., 218.),
            Self::Wall => Vec2::new(184., 186.),
        }
    }
}

/// Init resources related to the rendering of the map.
fn init_resources_for_rendering(mut commands: Commands) {
    commands.insert_resource(TilesGap(Vec2 { x: 70., y: 35. }));
    commands.insert_resource(TilesSize(Vec2 { x: 125., y: 100. }))
}

/// Renders the map.
fn render_map(
    query: Query<(Entity, &TilePosition, &Tile), Changed<Tile>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tiles_gap: Res<TilesGap>,
    tiles_size: Res<TilesSize>,
) {
    for (entity, position, tile) in query.iter() {
        let texture = tile.get_texture(&asset_server);

        let translation_2d = tiles_gap.0 * position.to_pixel_coordinates();
        let translation = Vec3::new(
            translation_2d.x,
            translation_2d.y,
            z_position_from_y(translation_2d.y),
        );

        let scale_2d = tiles_size.0 / tile.get_image_size();

        // the y scale is the same as the x scale to keep the aspect ratio.
        let scale = Vec3::new(scale_2d.x, scale_2d.x, 1.0);

        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture,
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            ..default()
        });
    }
}

/// A simple sigmoid function to convert y position to z position.
/// The return value is between 0 and 1.
fn z_position_from_y(y: f32) -> f32 {
    -1.0 / (1.0 + (-y * 110_f64.powi(-3) as f32).exp())
}
