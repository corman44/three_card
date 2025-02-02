use bevy::{prelude::*, utils::HashSet};
use rand::{seq::SliceRandom, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub number: CardVal,
    pub suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.number.partial_cmp(&other.number) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.suit.partial_cmp(&other.suit)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
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
            Self {
                number: CardVal::Two,
                suit: Suit::Club,
            }
        } else if self.to_num() > 52 {
            Self {
                number: CardVal::Ace,
                suit: Suit::Diamond,
            }
        } else {
            self
        }
    }
}

impl Card {
    pub fn from_u8(num: u8) -> Self {
        Card {
            number: {
                match num % 13 {
                    0 => CardVal::Two,
                    1 => CardVal::Three,
                    2 => CardVal::Four,
                    3 => CardVal::Five,
                    4 => CardVal::Six,
                    5 => CardVal::Seven,
                    6 => CardVal::Eight,
                    7 => CardVal::Nine,
                    8 => CardVal::Ten,
                    9 => CardVal::Jack,
                    10 => CardVal::Queen,
                    11 => CardVal::King,
                    12 => CardVal::Ace,
                    _ => panic!("impossible card selection")
                }
            },
            suit: {
                match num / 13 {
                    0 => Suit::Club,
                    1 => Suit::Heart,
                    2 => Suit::Spade,
                    _ => Suit::Diamond,
                }
                
            },
        }
    }

    pub fn to_num(&self) -> u8 {
        let mut out = 0;
        match self.suit {
            Suit::Heart => out += 13,
            Suit::Spade => out += 26,
            Suit::Diamond => out += 39,
            _ => {}
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

#[derive(Debug, Clone, Resource, Default)]
pub struct Pile {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct DeadCards {
    pub cards: HashSet<Card>,
}

#[derive(Debug, Clone, Component)]
pub struct PlayerTurnText;

#[derive(Component, Clone, Default, Debug)]
pub struct Player {
    pub handle: u64,
    pub facedown_cards: Vec<Card>,
    pub faceup_cards: Vec<Card>,
    pub hand: BTreeSet<Card>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    King = 11,
    Ace = 12,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub struct CardDeck {
    pub cards: CardGroup,
}

impl Default for CardDeck {
    fn default() -> Self {
        // ordered deck of cards
        let mut deck = Vec::new();

        for &suit in &[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for &number in &[
                CardVal::Two,
                CardVal::Three,
                CardVal::Four,
                CardVal::Five,
                CardVal::Six,
                CardVal::Seven,
                CardVal::Eight,
                CardVal::Nine,
                CardVal::Ten,
                CardVal::Jack,
                CardVal::Queen,
                CardVal::King,
                CardVal::Ace,
            ] {
                deck.push(Card { number, suit });
            }
        }

        Self {
            cards: CardGroup {
                cards: deck,
                location: CardLocation::Deck,
            },
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
pub struct SelectedCards {
    pub cards: HashSet<Card>,
}

impl SelectedCards {
    pub fn value(&self) -> Option<CardVal> {
        if let Some(card) = self.cards.iter().next() {
            Some(card.number)
        } else {
            None
        }
    }
}

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

#[derive(Clone, Debug, Default, States, Hash, PartialEq, Eq)]
pub enum PlayerTurnState {
    #[default]
    LocalPlayerTurn,
    RemotPlayerTurn(u64),
}

#[derive(Resource, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct PlayerTurn {
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
