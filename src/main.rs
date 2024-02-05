use bevy::{prelude::*, transform::commands};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Tank;

#[derive(Component, Debug)]
struct Position(Vec3);

#[derive(Component)]
struct Rotation(Quat);

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
        transform: Transform::from_xyz(4.0, 12.0, 4.0),
        ..default()
    });
}

fn read_keyboard_input(
    mut query: Query<(&mut Position,&mut Rotation, With<Tank>)>,
    keyboard_input: Res<Input<KeyCode>>,
    delta_time: Res<Time>,
) {
    for (mut position, mut rotation, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            position.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 10.0)) * delta_time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            position.0 -= rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 10.0)) * delta_time.delta_seconds();
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.5, 10.5, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    ass: Res<AssetServer>,
) {
    let model = ass.load("tank.glb#Scene0");
    commands.spawn((
        Tank,
        Position(Vec3::new(0.0, 0.0, 0.0)),
        Rotation(Quat::from_rotation_y(0.0)),
        SceneBundle {
            scene: model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
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