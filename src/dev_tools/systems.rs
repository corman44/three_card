use bevy::prelude::*;

use crate::{game::DeckState, AppState};


pub fn handle_info_timer(

) {
    
}

pub fn print_state_changes(
    deck_state: Res<State<DeckState>>,
    app_state: Res<State<AppState>>,
) {
    if deck_state.is_changed() {
        info!("Deck State::{:?}", deck_state.get());
    }

    if app_state.is_changed() {
        info!("AppState::{:?}", app_state.get());
    }
}
