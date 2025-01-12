use bevy::prelude::*;
use bevy_matchbox::{prelude::PeerId, MatchboxSocket};

use crate::{game::components::{LocalPlayers, Player, PlayerTurn}, AppState};

use super::{components::{ActionType, PlayerCommand}, SessionSeed};

pub fn start_matchbox_socket(
    mut commands: Commands,
) {
    let room_url = "ws://192.168.2.44:3536/three_card?next=2";
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
    let mut peer_count = 0;
    for peer in socket.connected_peers() {
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
    
    commands.insert_resource(LocalPlayers{ 0: vec![id.0 ^ id.1]});
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

pub fn rx_msg(
    mut socket: ResMut<MatchboxSocket>,
    local_player: Res<LocalPlayers>
) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }
    
    let mut channel = socket.get_channel_mut(0).expect("no channel 0..");
    for (id, msg) in channel.receive() {
        let decoded: PlayerCommand = bitcode::decode(&msg).expect("unable to decode PlayerCommand");
        match decoded.action {
            ActionType::PickupPile => {
                info!("msg from: {id}\nAction::PickupPile");
            },
            ActionType::PickupDeck => {
                info!("msg from: {id}\nAction::PickupDeck");
            },
            ActionType::PlayCards => {
                info!("msg from: {id}\nAction::PlayCards\nCards: {:?}",decoded.data.expect("no cards provided for PlayCards"));
            },
        }
    }
}

pub fn tx_msg(
    mut socket: ResMut<MatchboxSocket>,
    button: Res<ButtonInput<KeyCode>>,
) {
    if button.just_pressed(KeyCode::Digit1) { // PlayCards Command
        let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
        let channel = socket.get_channel_mut(0).expect("no channel 0..");
        let msg = PlayerCommand {
            action: ActionType::PlayCards,
            data: Some(vec![0,1,2]),
        };
        channel.send(bitcode::encode(&msg).into(), peer);
    }
    
    if button.just_pressed(KeyCode::Digit2) { // PickupDeck Command
        let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
        let channel = socket.get_channel_mut(0).expect("no channel 0..");
        let msg = PlayerCommand {
            action: ActionType::PickupDeck,
            ..default()
        };
        channel.send(bitcode::encode(&msg).into(), peer);
    }

    if button.just_pressed(KeyCode::Digit3) { // PickupPile Command
        let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
        let channel = socket.get_channel_mut(0).expect("no channel 0..");
        let msg = PlayerCommand {
            action: ActionType::PickupPile,
            ..default()
        };
        channel.send(bitcode::encode(&msg).into(), peer);
    }
}
