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

/// System to display debug tank data.
fn ui_example_system(mut contexts: EguiContexts, query: Query<(&Position, &Force, &Mass, &Rotation, &Velocity), With<Player>>) {
    egui::Window::new("Debug Tank Data").show(contexts.ctx_mut(), |ui| {
        for (position, force, mass, rotation, velocity) in query.iter() {
            ui.label(format!("Position: {:.2?}", position.0));
            ui.label(format!("Force: {:.2?}", force.0));
            ui.label(format!("Mass: {:.2?}", mass.0));
            ui.label(format!("Rotation: {:.2?}", rotation.0));
            ui.label(format!("Velocity: {:.2?}", velocity.0.length()));
        }
    });
}