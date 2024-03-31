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

/// The default ui layout size.
#[derive(Resource)]
pub struct UILayoutSize(pub Vec2);

/// Initializes [UILayoutSize].
pub fn init_window_size(mut command: Commands) {
    command.insert_resource(UILayoutSize(Vec2::new(1280., 720.)));
}

/// Calculates the ui_scale.0 depending on the [UILayoutSize]
/// in order to make the ui layout responsive.
pub fn change_scaling(
    mut ui_scale: ResMut<UiScale>,
    windows: Query<&Window>,
    size: Res<UILayoutSize>,
) {
    if !windows.get_single().unwrap().resolution.physical_height() > 0 {
        return;
    };
    let window = windows.get_single().expect("Main window not found");
    let (a, b) = (
        window.resolution.width() / size.0.x,
        window.resolution.height() / size.0.y,
    );
    ui_scale.0 = if a < b { a } else { b } as f64
}
