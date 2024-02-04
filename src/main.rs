use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Tank;

#[derive(Component, Debug)]
struct Position(Vec3);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            // WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (
            spawn_tank_default,
            spawn_camera,
            spawn_lights,
            spawn_floor,
        ))
        .add_systems(Update, (
            ui_example_system,  
            read_keyboard_input,
        ))
        .add_systems(PostUpdate, (
            update_model_pos,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
        ))
        .run();
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn read_keyboard_input(
    mut query: Query<(&mut Position, With<Tank>)>,
    keyboard_input: Res<Input<KeyCode>>,
    delta_time: Res<Time>,
) {
    for (mut position, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            position.0.z -= 1.0 * delta_time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            position.0.z += 1.0 * delta_time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            position.0.x -= 1.0 * delta_time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            position.0.x += 1.0 * delta_time.delta_seconds();
        }
    }
}

fn update_model_pos(
    mut query: Query<(&Position, &mut Transform), With<Tank>>
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = position.0;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0, subdivisions: 5})),
        material: materials.add(Color::rgb_u8(200, 200, 200).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

}

fn spawn_tank_default(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Tank,
        Position(Vec3::new(0.0, 0.5, 0.0)),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));
}

fn ui_example_system(mut contexts: EguiContexts, query: Query<(&Position, With<Tank>)>) {
    egui::Window::new("Debug Tank Data").show(contexts.ctx_mut(), |ui| {
        for (position, _) in query.iter() {
            ui.label(format!("{:?}", position));
        }
    });
}