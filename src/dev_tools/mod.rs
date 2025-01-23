pub mod component;
pub mod systems;

use bevy::prelude::*;
use component::DebugTimer;
use systems::{handle_debug_timer, print_all_info, print_networking_info, print_state_changes};

use crate::game::components::DeckState;


pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugTimer>()
            .add_systems(Update, (print_state_changes, print_networking_info, print_all_info))
            // .add_systems(Update, handle_debug_timer.run_if(
            //     in_state(DeckState::Display)
            //     .or(in_state(DeckState::Gameplay))
            // ))
            ;
    }
}
