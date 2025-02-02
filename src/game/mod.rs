pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{
    CardDeck, DeadCards, DeckState, Pile, PlayerTurn, PlayerTurnState, SelectedCards,
};
use systems::{
    deal_cards, display_table_cards, display_turn, monitor_lp_inputs, rx_other_players,
    select_cards, setup, update_player_turn_state,
};

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardDeck>()
            .init_resource::<DeadCards>()
            .init_resource::<PlayerTurn>()
            .init_resource::<Pile>()
            .init_resource::<SelectedCards>()
            .init_state::<DeckState>()
            .init_state::<PlayerTurnState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::PlayersMatched), deal_cards)
            .add_systems(OnEnter(DeckState::Dealt), (display_table_cards))
            .add_systems(
                Update,
                (
                    display_turn,
                    select_cards,
                    update_player_turn_state,
                    rx_other_players,
                    monitor_lp_inputs,
                )
                    .run_if(in_state(DeckState::Gameplay)),
            );
    }
}

/* Game States Flow
-> Players Matched kick everything off
 -> cards can be dealt
 -> DeckState::Dealt means the cards can now be properly displayed
 -> DeckState::Display
*/
