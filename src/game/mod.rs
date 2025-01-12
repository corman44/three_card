pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{CardDeck, DeckState, PlayerTurn};
use systems::{deal_cards, display_table_cards, display_turn, setup};

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CardDeck>()
            .init_resource::<PlayerTurn>()
            .init_state::<DeckState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::PlayersMatched), deal_cards)
            .add_systems(OnEnter(DeckState::Dealt), display_table_cards)
            .add_systems(Update, display_turn.run_if(
                in_state(DeckState::Display)
                .or(in_state(DeckState::Gameplay))
            ))
            ;
    }
}
