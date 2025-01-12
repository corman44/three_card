use bevy::prelude::*;
use bevy_matchbox::{matchbox_socket::{WebRtcChannel, WebRtcSocket}, MatchboxSocket};

use crate::{game::components::{DeckState, PlayerTurn}, AppState};

use super::component::DebugTimer;

pub fn handle_debug_timer(
    time: Res<Time>,
    mut info_timer: ResMut<DebugTimer>,
    mut player_turn_state: ResMut<PlayerTurn>,
) {
    if info_timer.0.tick(time.delta()).just_finished() {
        player_turn_state.next();
    }
}

pub fn print_state_changes(
    deck_state: Res<State<DeckState>>,
    app_state: Res<State<AppState>>,
    player_turn: Res<PlayerTurn>,
) {
    if deck_state.is_changed() {
        info!("Deck State::{:?}", deck_state.get());
    }

    if app_state.is_changed() {
        info!("AppState::{:?}", app_state.get());
    }

    if player_turn.is_changed() {
        info!("PlayerTurn: {:?}", player_turn);
    }
}

pub fn print_networking_info(
    socket: Res<MatchboxSocket>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    if buttons.just_pressed(KeyCode::KeyN) {
        let peers = socket.connected_peers().into_iter().collect::<Vec<_>>();
        let channel = socket.channel(0);
        let config = channel.config();
        info!("**** Networking Debug Info ****");
        info!("Channel: {:?}", channel);
        info!("ChannelConfig: {:?}", config);
        info!("Peers: {:?}", peers);

    }
}
