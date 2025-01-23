use bevy::prelude::*;
use bevy_matchbox::{matchbox_socket::{WebRtcChannel, WebRtcSocket}, MatchboxSocket};

use crate::{game::components::{DeadCards, DeckState, Pile, PlayerTurn}, networking::components::GameRoom, AppState};

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
    app_state: Res<State<AppState>>,
    deck_state: Res<State<DeckState>>,
    player_turn: Res<PlayerTurn>,
    pile: Res<Pile>,
    dead_cards: Res<DeadCards>,

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
    
    if pile.is_changed() {
        info!("Pile: {:?}", pile.cards);
    }
    
    if dead_cards.is_changed() {
        info!("DeadCards: {:?}", dead_cards.cards);
    }
}

pub fn print_networking_info(
    room: Res<GameRoom>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    if buttons.just_pressed(KeyCode::KeyN) {
        let peers = room.socket.connected_peers().into_iter().collect::<Vec<_>>();
        let channel = room.socket.channel(0);
        let config = channel.config();
        info!("**** Networking Debug Info ****");
        info!("Channel: {:?}", channel);
        info!("ChannelConfig: {:?}", config);
        info!("Peers: {:?}", peers);

    }
}

pub fn print_all_info(
    app_state: Res<State<AppState>>,
    buttons: Res<ButtonInput<KeyCode>>,
    dead_cards: Res<DeadCards>,
    deck_state: Res<State<DeckState>>,
    player_turn: Res<PlayerTurn>,
    pile: Res<Pile>,
    room: Res<GameRoom>
) {
    if buttons.pressed(KeyCode::ShiftLeft) && buttons.just_pressed(KeyCode::KeyA) {
        dbg!(&app_state);
        dbg!(&buttons);
        dbg!(&dead_cards);
        dbg!(&deck_state);
        dbg!(&player_turn);
        dbg!(&pile);
        dbg!(&room);
    }
}
