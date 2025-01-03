pub mod components;
pub mod systems;


use bevy::prelude::*;
use systems::{start_matchbox_socket, wait_for_players};

use crate::AppState;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players.run_if(in_state(AppState::WaitingForPlayers)))
        ;
    }
}

#[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
pub struct SessionSeed(u64);
