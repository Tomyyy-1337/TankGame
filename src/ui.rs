use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::physics::{Force, Mass, Position, Rotation, Velocity};
use crate::tank::Player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            ui_example_system,
        ));
    }
}

fn ui_example_system(mut contexts: EguiContexts, query: Query<(&Position, &Force, &Mass, &Rotation, &Velocity), With<Player>>) {
    egui::Window::new("Debug Tank Data").show(contexts.ctx_mut(), |ui| {
        for (position, force, mass, rotation, velocity) in query.iter() {
            ui.label(format!("Position: {:?}", position.0));
            ui.label(format!("Force: {:?}", force.0));
            ui.label(format!("Mass: {:?}", mass.0));
            ui.label(format!("Rotation: {:?}", rotation.0));
            ui.label(format!("Velocity: {}", velocity.0.length()));
        }
    });
}