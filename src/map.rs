use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.8, 0.8, 1.0)))
            .insert_resource(AmbientLight {
                color: Color::rgb(0.8, 0.8, 0.8),
                brightness: 0.75,
            })
            .add_systems(Startup, (
            spawn_floor,
        ));
    }
}

fn spawn_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0, subdivisions: 5})),
        material: materials.add(Color::rgb_u8(240, 240, 240).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
