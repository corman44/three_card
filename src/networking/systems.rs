use bevy::prelude::*;
use bevy_matchbox::MatchboxSocket;

use crate::{game::components::{LocalPlayers, Player, PlayerTurn}, AppState};

use super::{components::{ActionType, GameRoom, PlayerCommand}, SessionSeed};

pub fn start_matchbox_socket(
    mut commands: Commands,
) {
    let room_url = "ws://127.0.0.1:3536/three_card?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut next_matchmaking_state: ResMut<NextState<AppState>>,
    mut players_query: Query<&mut Player>,
    mut player_turn: ResMut<PlayerTurn>,
    mut socket: ResMut<MatchboxSocket>,
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
    commands.insert_resource(GameRoom {
        
    });
    commands.insert_resource(SessionSeed(seed));
    next_matchmaking_state.set(AppState::PlayersMatched);
}

pub fn rx_msg(
    mut socket: ResMut<MatchboxSocket>,
    local_player: Res<LocalPlayers>,
    mut player_turn: ResMut<PlayerTurn>,

) {
    // if other players turn then we listen for messages
    socket.update_peers();
    let channel = socket.get_channel_mut(0).expect("no channel 0 found..");
    for (id, msg) in channel.receive() {
        let decoded: PlayerCommand = bitcode::decode(&msg).expect("unable to decode PlayerCommand");
        match decoded.action {
            ActionType::PickupPile => {
                if *local_player.0.first().unwrap() != player_turn.current_turn() {
                    info!("msg from: {id}\nAction::PickupPile");
                    player_turn.next();
                }
            },
            ActionType::PickupDeck => {
                info!("msg from: {id}\nAction::PickupDeck");
            },
            ActionType::PlayCards => {
                if *local_player.0.first().unwrap() != player_turn.current_turn() {
                    info!("msg from: {id}\nAction::PlayCards\nCards: {:?}",decoded.data.expect("no cards provided for PlayCards"));
                    player_turn.next();
                }
            },
        }
    }
}

pub fn tx_msg(
    button: Res<ButtonInput<KeyCode>>,
    local_player: Res<LocalPlayers>,
    mut players_query: Query<&mut Player>,
    mut player_turn: ResMut<PlayerTurn>,
    mut socket: ResMut<MatchboxSocket>,
) {
    if *local_player.0.first().unwrap() == player_turn.current_turn() {

        if button.just_pressed(KeyCode::KeyC) { // PlayCards Command
            let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
            let channel = socket.get_channel_mut(0).expect("no channel 0..");
            let msg = PlayerCommand {
                action: ActionType::PlayCards,
                data: Some(vec![0,1,2]),
            };
            channel.send(bitcode::encode(&msg).into(), peer);
            player_turn.next();
        }
        
        if button.just_pressed(KeyCode::KeyP) { // PickupPile Command
            let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
            let channel = socket.get_channel_mut(0).expect("no channel 0..");
            let msg = PlayerCommand {
                action: ActionType::PickupPile,
                ..default()
            };
            channel.send(bitcode::encode(&msg).into(), peer);
            player_turn.next();
        }

    }

    if button.just_pressed(KeyCode::KeyD) { // PickupDeck Command
        // TODO check if LP Hand is < 3 then allow for card pickup
        for player in players_query.iter() {
            if player.handle == *local_player.0.first().unwrap() { // found LP
                if player.hand.len() <= 3 {
                    let peer = socket.connected_peers().into_iter().next().expect("no connected peers");
                    let channel = socket.get_channel_mut(0).expect("no channel 0..");
                    let msg = PlayerCommand {
                        action: ActionType::PickupDeck,
                        ..default()
                    };
                    channel.send(bitcode::encode(&msg).into(), peer);

                    // TODO pickup X cards

                }
            }
        }

    }
}
