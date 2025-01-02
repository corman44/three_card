use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{game::{DeckState, PlayerTurn}, AppState};

use super::component::InfoTimer;


pub fn handle_info_timer(
    time: Res<Time>,
    mut info_timer: ResMut<InfoTimer>,
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
        info!("PlayerTurn::{:?}", player_turn.0);
    }
}
