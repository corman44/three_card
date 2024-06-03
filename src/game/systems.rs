use bevy::prelude::*;

use crate::CardDeck;

use super::components::{Card, Player};

pub fn add_players(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle::default(),
        Player::default(),
    ));

    commands.spawn((
        NodeBundle::default(),
        Player::default(),
    ));
}

pub fn deal_cards(
    mut players: Query<&mut Player>,
    mut card_deck: ResMut<CardDeck>,
) {
    // for each player in the game, deal 3 cards facedown, 3 faceup, 3 to the hand
    for mut player in players.iter_mut() {
        player.facedown_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.facedown_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.facedown_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.faceup_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.faceup_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.faceup_cards.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.hand.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.hand.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
        player.hand.as_mut().unwrap().push(Card(card_deck.cards.pop().unwrap()));
    }
    info!("Cards Dealt: \n{players:?}");
}
