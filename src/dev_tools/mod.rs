pub mod component;
pub mod systems;

use bevy::prelude::*;
use component::InfoTimer;
use systems::{handle_info_timer, print_state_changes};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InfoTimer>()
            .add_systems(Update, print_state_changes)
            // .add_systems(Update, handle_info_timer)
            ;
    }
}
