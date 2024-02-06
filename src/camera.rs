use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};

use crate::{physics::{Position, Rotation}, tank::Player};
use crate::schedule::ScheduleSet;

pub struct CameraPlugin;

/// Zoom component for the camera.
/// The zoom is a f32 value that is used to determine the distance from the camera to the player.
#[derive(Component)]
pub struct Zoom(pub f32);

/// MousePosition component for the camera.
/// The mouse position is a Vec2 value that is used to determine the last position of the mouse.
#[derive(Component)]
pub struct MousePosition(pub Vec2);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_camera,
        ))
        .add_systems(Update, (
            update_camera_zoom,
            zoom_key,
        ).in_set(ScheduleSet::Input))
        .add_systems(Update, (
            update_camera,
        ).in_set(ScheduleSet::UpdateWorld));
    }
}

/// Spawn the camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.5, 10.5, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Zoom(40.0),
        MousePosition(Vec2::ZERO),
    ));
}

/// System to update the camera position and rotation to follow the player.
fn update_camera(
    query: Query<(&Position, &Rotation), With<Player>>, 
    mut camera_query: Query<(&mut Transform, &Zoom), With<Camera>>,
) {
    for (position, rotation) in query.iter() {
        for (mut transform, zoom) in camera_query.iter_mut() {
            transform.translation = position.0 + rotation.0.mul_vec3(Vec3::new(-0.0, 15.5, -zoom.0));
            transform.look_at(position.0 + (rotation.0.mul_vec3(Vec3::new(0.0, 5.0, (-10.0 * zoom.0).max(80.0)))), Vec3::Y);
        }
    }
}

// fn update_mouse_position(
//     mut mouse_position: Query<&mut MousePosition, With<Camera>>,
//     mut new_mouse_motion: ResMut<Events<MouseMotion>>,
// ) {
//     for ev in new_mouse_motion.drain() {
//         for mut mp in mouse_position.iter_mut() {
//             mp.0 += ev.delta;
//         }
//     }
// }

/// System to update the camera zoom based on the mouse wheel input.
fn update_camera_zoom(
    mut zoom: Query<&mut Zoom, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read(){
        if let MouseScrollUnit::Line = ev.unit {
            for mut z in zoom.iter_mut() {
                if z.0 == 30.0 && ev.y > 0.0 {
                    z.0 = -30.0;
                    continue;
                }
                if z.0 == -30.0 && ev.y < 0.0 {
                    z.0 = 30.0;
                    continue;
                }
                z.0 += ev.y * -10.0;
            }
        }
    }
}

/// System to update the camera zoom based on the keyboard input
/// Switch the zoom between third person and first person
fn zoom_key (
    mut zoom: Query<&mut Zoom, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut z in zoom.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
            if z.0 <= -30.0 {
                z.0 = 50.0;
            } else if z.0 >= 30.0 {
                z.0 = -40.0;
            }
        }
    }
}

