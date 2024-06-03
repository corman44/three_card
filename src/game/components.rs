use bevy::prelude::*;

use crate::CardVal;

#[derive(Debug, Component, Default)]
pub struct Active(pub bool);

#[derive(Debug, Clone, Component, Default)]
pub struct Card(pub CardVal);

#[derive(Bundle, Clone, Default)]
pub struct CardBundle {
    card: Card,
    sprite: SpriteBundle,
}

#[derive(Bundle, Default)]
pub struct UICardBundle {
    active: Active,
    card_bundle: CardBundle,
}

#[derive(Component, Default)]
pub struct Player {
    pub facedown_cards: Option<Vec<Card>>,
    pub faceup_cards: Option<Vec<Card>>,
    pub hand: Option<Vec<Card>>,
}