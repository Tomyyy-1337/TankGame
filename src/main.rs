use bevy::{prelude::*, window::PresentMode};

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

pub mod menu;
use menu::MenuPlugin;

pub mod schedule;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TankGame".into(),
                resolution: (1000., 750.).into(),
                present_mode: PresentMode::Mailbox,
                fit_canvas_to_parent: true,
                ..default()
                }),
            ..default()
            }),
            bevy_framepace::FramepacePlugin,
            bevy_egui::EguiPlugin,
            AssetLoaderPlugin,
            MapPlugin,
            CameraPlugin,
            TankPlugin,
            UIPlugin,
            PhysicsPlugin,
            MenuPlugin,
            SchedulePlugin,
        ))
        .run();
}