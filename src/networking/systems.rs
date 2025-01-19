use bevy::prelude::*;
use bevy_matchbox::MatchboxSocket;

use crate::{
    game::components::{LocalPlayers, Player, PlayerTurn},
    AppState,
};

use super::{
    components::{GameRoom, IntoU64},
    SessionSeed,
};

pub fn create_room(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/three_card?next=2";
    info!("connecting to matchbox server: {room_url}");

    commands.insert_resource(GameRoom::new(MatchboxSocket::new_reliable(room_url), 0));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut next_matchmaking_state: ResMut<NextState<AppState>>,
    mut players_query: Query<&mut Player>,
    mut player_turn: ResMut<PlayerTurn>,
    mut room: ResMut<GameRoom>,
) {
    if room.socket.get_channel(0).is_err() {
        info!("socket error: {:?}", room.socket);
        return;
    }

    room.socket.update_peers();
    let peers = 1;
    if room.socket.connected_peers().count() == peers {
        info!("2 Players Connected :)");
    } else {
        return;
    }

    let id = room
        .socket
        .id()
        .expect("no peer id assigned")
        .0
        .as_u64_pair();
    let mut players: Vec<u64> = vec![id.0 ^ id.1];
    let mut seed = id.0 ^ id.1;
    let mut peer_count = 0;
    for peer in room.socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
        players.push(peer_id.0 ^ peer_id.1);
        peer_count += 1;
    }

    // Setup Player Turns
    players.sort();

    // Set Player IDs in Player Struct
    for (mut playa, id) in players_query.iter_mut().zip(players.iter()) {
        playa.handle = *id;
    }

    // Set Player IDs in PlayerTurn
    for playa in players {
        player_turn.ids.push(playa);
    }

    commands.insert_resource(LocalPlayers {
        0: vec![id.0 ^ id.1],
    });
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}
