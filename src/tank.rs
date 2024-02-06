use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, physics::{Force, Mass, Physics, Position, Rotation, Velocity}};
use crate::schedule::ScheduleSet;

/// Marker component for Tanks
#[derive(Component)]
pub struct Tank;

/// Marker component for the player
#[derive(Component)]
pub struct Player;

/// Plugin for the tank system.
pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_player_tank,
        ))
        .add_systems(Update, (
            player_tank_movement_input,
            slowdown_player_tank,
        ).in_set(ScheduleSet::Input))
        .add_systems(Update, (
            update_model_pos,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
        ).in_set(ScheduleSet::UpdateWorld));
    }
}

/// System to handle the player tank movement input.
fn player_tank_movement_input (
    mut query: Query<(&mut Rotation, &mut Force, &Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    delta_time: Res<Time>,
) {
    for (mut rotation, mut force, velocity) in query.iter_mut() {
        if velocity.0.length() > 2.5 {
            if keyboard_input.pressed(KeyCode::A) {
                // if driving forward, turn left else turn right
                force.0 += rotation.0.mul_vec3(Vec3::new(200.0 + velocity.0.length() * 2.0, 0.0, 0.0));
            }
            if keyboard_input.pressed(KeyCode::D) {
                force.0 += rotation.0.mul_vec3(Vec3::new(-200.0 - velocity.0.length() * 2.0, 0.0, 0.0));
            }
        } else {
            if keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::W) && !keyboard_input.pressed(KeyCode::S) {
                rotation.0 *= Quat::from_rotation_y(0.75 * delta_time.delta_seconds());
            }
            if keyboard_input.pressed(KeyCode::D) && !keyboard_input.pressed(KeyCode::W) && !keyboard_input.pressed(KeyCode::S){
                rotation.0 *= Quat::from_rotation_y(-0.75 * delta_time.delta_seconds());
            }
        }
        
        if keyboard_input.pressed(KeyCode::W) {
            force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 5000.0));
        }
        if keyboard_input.pressed(KeyCode::S) {
            force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, -4000.0));
        }
    }
}

/// System to slow down the player tank while moving.
fn slowdown_player_tank (
    mut query: Query<(&mut Force, &Velocity, &Mass), With<Player>>,
) {
    for (mut force, velocity, mass) in query.iter_mut() {
        if velocity.0.length() > 2.5 {
            force.0 -= velocity.0 * 2.0 * mass.0;
        }
    }
}

/// System to update the model position based on the physics position and rotation.
fn update_model_pos(
    mut query: Query<(&Position, &Rotation, &mut Transform), With<Tank>>
) {
    for (position, rotation, mut transform) in query.iter_mut() {
        transform.translation = position.0;
        transform.rotation = rotation.0;
    }
}

/// System to spawn the player tank.
/// This system is run once at the start of the game.
/// It spawns the player tank with the `Tank`, `Player`, 'Physics', and `SceneBundle` components.
fn spawn_player_tank (
    mut commands: Commands,
    assets: Res<SceneAssets>
) {
    commands.spawn((
        Tank,
        Player,
        Physics {
            mass: Mass(100.0),
            ..Default::default()
        },
        SceneBundle {
            scene: assets.tank.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}