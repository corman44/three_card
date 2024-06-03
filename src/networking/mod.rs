pub mod components;
pub mod systems;

use std::default;

use bevy::prelude::*;
use systems::{start_matchbox_socket, wait_for_players};

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players)
        ;
    }
}

