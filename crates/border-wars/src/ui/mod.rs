//! The file that contains the UI logic.

pub mod hover;
pub mod responsive_scale;

use bevy::prelude::*;

use self::hover::HoverPlugin;
use self::responsive_scale::ResponsiveScalingPlugin;

/// The plugin for the UI.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HoverPlugin)
            .add_plugins(ResponsiveScalingPlugin);
    }
}
