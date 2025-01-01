pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::{Card, CardGroup};
use rand::{seq::SliceRandom, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use systems::{deal_cards, display_table_cards, display_turn, setup, short_wait};

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CardDeck>()
            .init_state::<DeckState>()
            .init_state::<PlayerTurn>()
            .add_systems(OnEnter(AppState::PlayersMatched), setup)
            .add_systems(OnEnter(AppState::GameStart), deal_cards.after(short_wait))
            .add_systems(Update, short_wait.run_if(in_state(DeckState::Shuffled)))
            .add_systems(OnEnter(DeckState::Dealt), display_table_cards)
            .add_systems(Update, display_turn.run_if(in_state(DeckState::Display)))
            ;
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub enum DeckState {
    #[default]
    Ordered,
    Shuffled,
    Dealt,
    Gameplay,
    Display,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardVal {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 12,
    Ace = 13,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Club,
    Heart,
    Spade,
    Diamond,
}

#[derive(Clone, Copy, Debug)]
pub enum CardLocation {
    LPHand,
    LPFaceUp,
    RPHand,
    RPFaceUp,
    Deck,
    Dead,
    Pile,
}

#[derive(Resource, Debug, Clone)]
pub struct CardDeck{
    pub cards: CardGroup,
}

impl Default for CardDeck {
    fn default() -> Self {
        // ordered deck of cards
        let mut deck = Vec::new();

        for &suit in &[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for &number in &[
                CardVal::Two, CardVal::Three, CardVal::Four, CardVal::Five,
                CardVal::Six, CardVal::Seven, CardVal::Eight, CardVal::Nine,
                CardVal::Ten, CardVal::Jack, CardVal::Queen, CardVal::King, CardVal::Ace
            ] {
                deck.push(Card { number, suit });
            }
        }

        Self {
            cards: CardGroup {
                cards: deck,
                location: CardLocation::Deck,
            }
        }
    }
}

impl CardDeck {
    pub fn shuffle(&mut self, seed: u64) {
        // let mut rng = thread_rng().next_u64();
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        self.cards.cards.shuffle(&mut rng);
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct PlayerTurn(pub usize);

impl PlayerTurn {
    // only for 2 player atm
    pub fn next(mut self) {
        self.0 = self.0 ^ 1;
    }
}
