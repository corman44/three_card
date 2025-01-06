use bevy::prelude::*;
use bevy_matchbox::{prelude::PeerId, MatchboxSocket};

use crate::AppState;

use super::SessionSeed;

pub fn start_matchbox_socket(
    mut commands: Commands,
) {
    let room_url = "ws://127.0.0.1:3536/three_card?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_matchmaking_state: ResMut<NextState<AppState>>,
) {
    if socket.get_channel(0).is_err() {
        info!("socket error: {:?}", socket);
        return;
    }

    socket.update_peers();
    let players = socket.connected_peers();
    let num_players = 2;
    if players.count() < num_players {
        return;
    }
    info!("2 Players Connected :)");

    // TODO Create WebRtcSocket with expected messages
    let channel = socket.take_channel(0).unwrap();

    let id = socket.id().expect("no peer id assigned").0.as_u64_pair();
    let mut seed = id.0 ^ id.1;
    for peer in socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
    }
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

