use std::cmp::Ordering;
use bevy::prelude::*;
use super::{CardLocation, CardVal, Suit};

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
    pub handle: usize,
    pub facedown_cards: Vec<Card>,
    pub faceup_cards: Vec<Card>,
    pub hand: Vec<Card>,
}