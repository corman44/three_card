pub mod components;
pub mod systems;

use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use systems::{deal_cards, display_table_cards, setup_cards};

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CardDeck>()
            .init_state::<DeckState>()
            .add_systems(OnEnter(AppState::PlayersMatched), setup_cards)
            .add_systems(OnEnter(AppState::GameStart), deal_cards)
            .add_systems(OnEnter(DeckState::Shuffled), display_table_cards)
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
}

#[derive(Clone, Copy, Debug)]
pub enum CardVal {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Heart,
    Diamond,
    Spade,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct  Card {
    number: CardVal,
    suit: Suit,
}

#[derive(Resource, Debug, Clone)]
pub struct CardDeck{
    pub cards: Vec<Card>,
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
            cards: deck
        }
    }
}

impl CardDeck {
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}