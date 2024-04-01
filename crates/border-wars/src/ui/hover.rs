//! The file that contains the hover logic.

use bevy::prelude::*;

/// The plugin for the hover system.
pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hovering);
    }
}

/// A component that stores the hover texture and the original texture.
#[derive(Component, Clone)]
pub struct HoveredTexture {
    /// The original texture.
    pub texture: Handle<Image>,

    /// The hovered texture.
    pub hovered_texture: Handle<Image>,
}

/// The system that applies the hover logic by changing the texture.
fn hovering(
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
