use bevy::prelude::*;

use crate::menu::MenuState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ScheduleSet {
    CheckMenu,
    Input,
    Physics,
    UpdateWorld,   
    Debug, 
    PauseMenu
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            ScheduleSet::CheckMenu,
            (
                ScheduleSet::Input,
                ScheduleSet::Physics,
                ScheduleSet::UpdateWorld,
            ).chain().run_if(in_state(MenuState::Closed)),
            ScheduleSet::PauseMenu.run_if(in_state(MenuState::Open)),
            ScheduleSet::Debug,
        ).chain());
    }
}