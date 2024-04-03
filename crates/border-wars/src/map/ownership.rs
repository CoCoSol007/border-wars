//! All code related to the ownership of the tiles.

use bevy::prelude::*;

use crate::Player;

/// The owner of a tile.
#[derive(Component, Clone)]
pub struct Owner(pub Player);

/// The plugin to render the ownership of the tiles.
pub struct OwnershipPlugin;

impl Plugin for OwnershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_ownership);
        app.add_systems(Startup, setup_ownership_resources);
    }
}

/// The contrast of the ownership colors.
///
/// The value is a number between 0 and 1.
#[derive(Resource)]
pub struct OwnershipColorContrast(pub f32);

/// Init resources related to the ownership of the tiles.
fn setup_ownership_resources(mut commands: Commands) {
    commands.insert_resource(OwnershipColorContrast(0.4));
}

/// Render the ownership of the tiles by applying colors.
fn render_ownership(
    mut query: Query<(&mut Sprite, &Owner), (Changed<Owner>, Changed<Sprite>)>,
    contrast: Res<OwnershipColorContrast>,
) {
    for (mut sprite, owner) in query.iter_mut() {
        println!("{:?}", contrast.0);
        let (r, g, b) = owner.0.color;
        let target = mix_colors(Color::rgb_u8(r, g, b), sprite.color, 1. - contrast.0);

        sprite.color = target;
    }
}

/// Mixes two colors.
fn mix_colors(color1: Color, color2: Color, alpha: f32) -> Color {
    let [r1, g1, b1, _] = color1.as_rgba_u8();
    let [r2, g2, b2, _] = color2.as_rgba_u8();
    let mixed_r = (1.0 - alpha).mul_add(r1 as f32, alpha * r2 as f32).round() as u8;
    let mixed_g = (1.0 - alpha).mul_add(g1 as f32, alpha * g2 as f32).round() as u8;
    let mixed_b = (1.0 - alpha).mul_add(b1 as f32, alpha * b2 as f32).round() as u8;
    Color::rgb_u8(mixed_r, mixed_g, mixed_b)
}
