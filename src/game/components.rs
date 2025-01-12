use std::cmp::Ordering;
use bevy::prelude::*;
use rand::{seq::SliceRandom, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct  Card {
    pub number: CardVal,
    pub suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.to_num().partial_cmp(&other.to_num()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.suit.partial_cmp(&other.suit)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_num().cmp(&other.to_num())
    }
    
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }
    
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
    
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self.to_num() < 0 {
            Self { number: CardVal::Two, suit: Suit::Club}
        } else if self.to_num() > 52 {
            Self { number: CardVal::Ace, suit: Suit::Diamond}
        } else {
            self
        }
    }
}

impl Card {
    pub fn to_num(&self) -> u8 {
        let mut out = 0;
        match self.suit {
            Suit::Heart => out += 13,
            Suit::Spade => out += 26,
            Suit::Diamond => out += 39,
            _ => {},
        }
        out += self.number as u8;
        out
    }
}


#[derive(Resource)]
pub struct ShortWait {
    pub timer: Timer,
}

#[derive(Debug, Clone)]
pub struct CardGroup {
    pub cards: Vec<Card>,
    pub location: CardLocation,
}

#[derive(Debug, Component, Default)]
pub struct Active(pub bool);

#[derive(Debug, Clone, Component)]
pub struct LPTableCards(pub u8);

#[derive(Debug, Clone, Component)]
pub struct LPHandCards(pub u8);

#[derive(Debug, Clone, Component)]
pub struct RPTableCards(pub u8);

#[derive(Debug, Clone, Component)]
pub struct RPHandCards(pub u8);

#[derive(Debug, Clone, Component)]
pub struct Deck;

#[derive(Debug, Clone, Component)]
pub struct Pile;

#[derive(Debug, Clone, Component)]
pub struct PlayerTurnText;

#[derive(Debug, Clone, Component)]
pub struct DeadCards {
    cards: Vec<Card>,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Player {
    pub handle: u64,
    pub facedown_cards: Vec<Card>,
    pub faceup_cards: Vec<Card>,
    pub hand: Vec<Card>,
}

#[derive(Debug, Resource)]
pub struct LocalPlayers(pub Vec<u64>);

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

#[derive(Debug, Default, Resource)]
pub struct SelectedCards(pub Vec<u8>);

pub trait KeyToDigit {
    fn to_digit(&self) -> u8;
}

impl KeyToDigit for KeyCode {
    fn to_digit(&self) -> u8 {
        match *self {
            KeyCode::Digit0 => 0,
            KeyCode::Digit1 => 1,
            KeyCode::Digit2 => 2,
            KeyCode::Digit3 => 3,
            KeyCode::Digit4 => 4,
            KeyCode::Digit5 => 5,
            KeyCode::Digit6 => 6,
            KeyCode::Digit7 => 7,
            KeyCode::Digit8 => 8,
            KeyCode::Digit9 => 9,
            _ => panic!("Problems.."),
        }
    }
}

#[derive(Resource, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct PlayerTurn{
    pub ids: Vec<u64>,
    pub turn: usize,
}
impl PlayerTurn {
    pub fn next(&mut self) {
        self.turn += 1;
        if self.turn >= self.ids.len() {
            self.turn = 0;
        }
    }

    pub fn current_turn(&self) -> u64 {
        self.ids[self.turn]
    }
}
