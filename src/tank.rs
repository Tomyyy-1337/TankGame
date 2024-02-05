use bevy::prelude::*;

use crate::physics::{Force, Mass, Physics, Position, Rotation, Velocity};

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Player;

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_player_tank,
        ))
        .add_systems(PreUpdate, (
            read_keyboard_input,
        ))
        .add_systems(PostUpdate, (
            update_model_pos,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
        ));
    }
}


fn read_keyboard_input(
    mut query: Query<(&mut Rotation, &mut Force, &Velocity, &Mass), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    delta_time: Res<Time>,
) {
    for (mut rotation, mut force, velocity, mass) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            if velocity.0.length() < 5.0 {
                force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 2000.0));
            }
            force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 2000.0));
        }
        if keyboard_input.pressed(KeyCode::S) {
            if velocity.0.length() < 5.0 {
                force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, -2000.0));
            }
            force.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, -2000.0));
        }

        if velocity.0.length() > 5.0 {
            force.0 -= velocity.0 * 2.0 * mass.0;
            continue;
        }
        if keyboard_input.pressed(KeyCode::A) {
            rotation.0 *= Quat::from_rotation_y(2.0 * delta_time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::D) {
            rotation.0 *= Quat::from_rotation_y(-2.0 * delta_time.delta_seconds());
        }
        
    }
}

fn update_model_pos(
    mut query: Query<(&Position, &Rotation, &mut Transform), With<Tank>>
) {
    for (position, rotation, mut transform) in query.iter_mut() {
        transform.translation = position.0;
        transform.rotation = rotation.0;
    }
}

fn spawn_player_tank (
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let model = ass.load("tank.glb#Scene0");
    commands.spawn((
        Tank,
        Player,
        Physics {
            mass: Mass(100.0),
            ..Default::default()
        },
        SceneBundle {
            scene: model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}