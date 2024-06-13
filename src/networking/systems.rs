use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::{matchbox_socket::SingleChannel, MatchboxSocket};

use crate::{AppState, Config};

use super::SessionSeed;

pub fn start_matchbox_socket(
    mut commands: Commands,
) {
    let room_url = "ws://127.0.0.1:3536/three_card?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_matchmaking_state: ResMut<NextState<AppState>>,
) {
    if socket.get_channel(0).is_err() {
        info!("socket error: {:?}", socket);
        return;
    }

    socket.update_peers();
    let players = socket.players();
    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("2 Players Connected :D");

    // create GGRS P2P sesh
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player..");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session..");

    let id = socket.id().expect("no peer id assigned").0.as_u64_pair();
    let mut seed = id.0 ^ id.1;
    for peer in socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
    }
    commands.insert_resource(SessionSeed(seed));

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

