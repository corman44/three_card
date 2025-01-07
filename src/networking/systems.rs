use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use bevy_matchbox::{prelude::PeerId, MatchboxSocket};

use crate::{game::LocalPlayers, AppState};

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
    let peers = 1;
    if socket.connected_peers().count() == peers {
        info!("2 Players Connected :)");
    } else {
        return;
    }

    let id = socket.id().expect("no peer id assigned").0.as_u64_pair();
    let mut players: Vec<u64> = vec![id.0 ^ id.1];
    let mut seed = id.0 ^ id.1;
    for peer in socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
        players.push(peer_id.0 ^ peer_id.1);
    }
    
    // FIXME need to assign LocalPlayer and All Player IDs
    commands.insert_resource(LocalPlayers{ 0: vec![id.0 ^ id.1]});
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

