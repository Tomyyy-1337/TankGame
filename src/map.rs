use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

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

/// System to spawn the floor. The floor is a 40x40 grid of tiles centered at the origin.
fn spawn_floor (
    mut commands: Commands,
    assets: Res<SceneAssets>,
) {
    for i in -20..=20 {
        for j in -20..=20 {
            commands.spawn((
                SceneBundle {
                    scene: assets.floor.clone(),
                    transform: Transform::from_xyz(40.0 * i as f32, 0.0, 40.0 * j as f32),
                    ..Default::default()
                },
            ));
        }
    }
}