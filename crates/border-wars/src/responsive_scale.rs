//! The file that contains the responsive scaling logic.

use bevy::prelude::*;

/// The plugin for the responsive scaling.
pub struct ResponsiveScalingPlugin;

impl Plugin for ResponsiveScalingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_window_size);
        app.add_systems(Update, change_scaling);
    }
}

/// The default window size.
#[derive(Resource)]
pub struct WindowSize(pub Vec2);

/// Initializes the window size.
pub fn init_window_size(mut command: Commands) {
    command.insert_resource(WindowSize(Vec2::new(1280., 720.)));
}

/// Calculates the ui_scale.0 depending on the default screen size
/// in order to make the screen responsive.
pub fn change_scaling(
    mut ui_scale: ResMut<UiScale>,
    windows: Query<&Window>,
    size: Res<WindowSize>,
) {
    let window = windows.get_single().expect("Main window not found");
    let (a, b) = (
        window.resolution.width() / size.0.x,
        window.resolution.height() / size.0.y,
    );
    ui_scale.0 = if a < b { a } else { b } as f64
}
