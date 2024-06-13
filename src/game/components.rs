use bevy::prelude::*;

use super::Card;

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
pub struct DeadCards {
    cards: Option<Vec<Card>>,
}

#[derive(Bundle, Clone)]
pub struct CardBundle {
    card: Card,
    sprite: SpriteBundle,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Player {
    pub handle: usize,
    pub facedown_cards: Option<Vec<Card>>,
    pub faceup_cards: Option<Vec<Card>>,
    pub hand: Option<Vec<Card>>,
}