use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::{physics::{Position, Rotation}, tank::Player};

pub struct CameraPlugin;

#[derive(Component)]
pub struct Zoom(pub f32);


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_camera,
        ))
        .add_systems(PreUpdate, (
            update_camera_zoom,
        ))
        .add_systems(Update, (
            update_camera,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.5, 10.5, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Zoom(40.0),
    ));
}

fn update_camera(
    query: Query<(&Position, &Rotation), With<Player>>, 
    mut camera_query: Query<(&mut Transform, &Zoom), With<Camera>>,
) {
    for (position, rotation) in query.iter() {
        for (mut transform, zoom) in camera_query.iter_mut() {
            transform.translation = position.0 + rotation.0.mul_vec3(Vec3::new(-0.0, 15.5, -zoom.0));
            transform.look_at(position.0 + (rotation.0.mul_vec3(Vec3::new(0.0, 5.0, 60.0))), Vec3::Y);
        }
    }
}

fn update_camera_zoom (
    mut zoom: Query<&mut Zoom, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read(){
        if let MouseScrollUnit::Line = ev.unit {
            for mut z in zoom.iter_mut() {
                z.0 += ev.y * -10.0;
            }
        }
    }
}