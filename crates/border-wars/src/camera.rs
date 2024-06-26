//! This module contains the camera systems responsible for movement and
//! scaling.

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::CurrentScene;

/// The speed of camera movement.
#[derive(Resource)]
struct CameraSpeedMouvement(f32);

/// The speed of camera scaling.
#[derive(Resource)]
struct CameraSpeedScale(f32);

/// The minimum scale of the camera.
#[derive(Resource)]
struct MinimumScale(f32);

/// The maximum scale of the camera.
#[derive(Resource)]
struct MaximumScale(f32);

/// Key settings for camera movement.
#[derive(Resource)]
pub struct KeysMovementSettings {
    /// Key to move the camera up.
    pub up: KeyCode,

    /// Key to move the camera down.
    pub down: KeyCode,

    /// Key to move the camera right.
    pub right: KeyCode,

    /// Key to move the camera left.
    pub left: KeyCode,
}

/// A Bevy plugin for the camera.
/// Allows camera movement with the keyboard and scaling with the mouse.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(Startup, init_resources_for_camera)
            .add_systems(
                Update,
                (keyboard_movement_system, mouse_movement_system)
                    .run_if(in_state(CurrentScene::Game)),
            )
            .add_systems(Update, scale_system.run_if(in_state(CurrentScene::Game)));
    }
}

/// Initializes the camera.
fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Initializes the resources related to the camera.
///
/// - [KeysMovementSettings]: The key settings for camera movement.
/// - [CameraSpeedMouvement]: The speed of camera movement.
/// - [CameraSpeedScale]: The speed of camera scaling.
/// - [MinimumScale]: The minimum scale of the camera.
/// - [MaximumScale]: The maximum scale of the camera.
fn init_resources_for_camera(mut commands: Commands) {
    commands.insert_resource(KeysMovementSettings {
        up: KeyCode::Z,
        down: KeyCode::S,
        right: KeyCode::D,
        left: KeyCode::Q,
    });

    commands.insert_resource(CameraSpeedMouvement(400.0));
    commands.insert_resource(CameraSpeedScale(0.1));
    commands.insert_resource(MinimumScale(0.1));
    commands.insert_resource(MaximumScale(10.0));
}

/// Moves the camera with keyboard input.
fn keyboard_movement_system(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    keys_settings: Res<KeysMovementSettings>,
    movement_speed: Res<CameraSpeedMouvement>,
    delta_time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut dx = 0.0;
        let mut dy = 0.0;
        for key in keys.get_pressed() {
            match *key {
                up if up == keys_settings.up => dy += movement_speed.0,
                down if down == keys_settings.down => dy -= movement_speed.0,
                right if right == keys_settings.right => dx += movement_speed.0,
                left if left == keys_settings.left => dx -= movement_speed.0,
                _ => continue,
            }
        }

        transform.translation.x += dx * delta_time.delta_seconds();
        transform.translation.y += dy * delta_time.delta_seconds();
    }
}

/// Moves the camera with mouse input.
fn mouse_movement_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    windows: Query<&Window>,
    mut last_position: Local<Option<Vec2>>,
) {
    let window = windows.get_single().expect("Main window not found");
    let Some(position) = window.cursor_position() else {
        return;
    };

    if mouse_button_input.just_pressed(MouseButton::Right) {
        *last_position = Some(position);
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        *last_position = None;
    }

    if let Some(old_position) = *last_position {
        for (mut transform, projection) in query.iter_mut() {
            let offset = (old_position - position).extend(0.0) * Vec3::new(1., -1., 1.);
            transform.translation += offset * projection.scale;
        }
        *last_position = Some(position);
    }
}

/// Scales the view with mouse input.
fn scale_system(
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    min_scale: Res<MinimumScale>,
    max_scale: Res<MaximumScale>,
    scale_speed: Res<CameraSpeedScale>,
) {
    for event in scroll_event.read() {
        for mut projection in query.iter_mut() {
            if event.unit != MouseScrollUnit::Line {
                return;
            }

            let future_scale = event.y.mul_add(-scale_speed.0, projection.scale);
            if min_scale.0 < future_scale && future_scale < max_scale.0 {
                projection.scale = future_scale;
            }
        }
    }
}
