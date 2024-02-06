use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod map;
use map::MapPlugin;

mod camera;
use camera::CameraPlugin;

mod physics;
use physics::PhysicsPlugin;

mod tank;
use tank::TankPlugin;

mod ui;
use ui::UIPlugin;

mod asset_loader;
use asset_loader::AssetLoaderPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            AssetLoaderPlugin,
            MapPlugin,
            CameraPlugin,
            TankPlugin,
            UIPlugin,
            PhysicsPlugin,
        ))
        .run();
}