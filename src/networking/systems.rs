use bevy::prelude::*;
use bevy_matchbox::MatchboxSocket;

use crate::{game::components::{LocalPlayers, Player, PlayerTurn}, AppState};

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
    mut players_query: Query<&mut Player>,
    mut player_turn: ResMut<PlayerTurn>,
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
    
    // Set Player IDs in Player Struct
    for (mut playa, id) in players_query.iter_mut().zip(players.iter()) {
        playa.handle = *id;
    }
    info!("networking player_query: {:?}",players_query.iter().collect::<Vec<_>>());

    // Setup Player Turns
    players.sort();
    for playa in players {
        player_turn.ids.push(playa);
    }
    
    commands.insert_resource(LocalPlayers{ 0: vec![id.0 ^ id.1]});
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

