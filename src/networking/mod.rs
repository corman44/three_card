pub mod components;
pub mod systems;


use bevy::prelude::*;
use systems::{rx_msg, start_matchbox_socket, tx_msg, wait_for_players};

use crate::AppState;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players.run_if(in_state(AppState::WaitingForPlayers)))
            .add_systems(Update, (tx_msg,rx_msg).run_if(in_state(AppState::PlayersMatched)))
        ;
    }
}

#[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
pub struct SessionSeed(u64);
