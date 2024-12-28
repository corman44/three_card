
use bevy::prelude::*;
use bevy_ggrs::{LocalPlayers, PlayerInputs};

use crate::{networking::SessionSeed, AppState, Config};

use super::{components::{Card, Deck, LPHandCards, LPTableCards, Player, RPHandCards, RPTableCards}, CardDeck, CardVal, DeckState, Suit};

pub const CARD_LOCATION: &str = r"normal_cards\individual\";
pub const CARD_BACK_LOACTION: &str = r"normal_cards\individual\card back\cardBackGreen.png";
pub const CARD_SCALE: f32 = 0.1;

pub fn setup_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {

    // spawn player1
    commands.spawn((
        Node::default(),
        Player {
            handle: 0,
            ..default()
        },
    ));

    
    // spawn player2
    commands.spawn((
        Node::default(),
        Player {
            handle: 1,
            ..default()
        },
    ));

    // Spawn Cards..
    for idx in 0..3 {
        // LP Card
        commands.spawn((
            Sprite {
                image: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
                ..default()
            },
            Visibility::Hidden,
            Transform {
                translation: Vec3::new(0. + 30. * idx as f32, 0., 1.),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..default()
            },
            LPHandCards(idx),
        ));

        // LP Faceup
        commands.spawn((
            Sprite {
                image: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
                ..default()
            },
            Visibility::Hidden,
            Transform {
                translation: Vec3::new(-155. + 20. * idx as f32, -190., 1.),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..default()
            },
            LPTableCards(idx)
        ));

        // RP Faceup
        commands.spawn((
            Sprite {
                image: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
                ..default()
            },
            Visibility::Hidden,
            Transform {
                translation: Vec3::new(155. + 20. * idx as f32, 200., 1.),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..default()
            },
            RPTableCards(idx)
        ));

        // RP Card
        commands.spawn(RPHandCards(idx));
    }

    // Spawn Deck Card
    commands.spawn((
        Sprite {
            image: asset_server.load(r"normal_cards\individual\card back\cardBackGreen.png"),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        Deck
    ));

    next_app_state.set(AppState::GameStart);
}

pub fn deal_cards(
    mut players: Query<&mut Player>,
    mut card_deck: ResMut<CardDeck>,
    mut next_deck_state: ResMut<NextState<DeckState>>,
    sesh_seed: Res<SessionSeed>,
) {
    // first shuffle the deck
    card_deck.shuffle(**sesh_seed);

    // for each player in the game, deal 3 cards facedown, 3 faceup, 3 to the hand
    for mut player in players.iter_mut() {
        player.facedown_cards = vec![card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap()];
        player.faceup_cards = vec![card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap()];
        player.hand = vec![card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap(), card_deck.cards.cards.pop().unwrap()];
    }
    next_deck_state.set(DeckState::Shuffled);
}

pub fn display_table_cards(
    asset_server: Res<AssetServer>,
    local_players: Res<LocalPlayers>,
    player_query: Query<&Player>,
    mut lp_tablecards_image_query: Query<(&mut Sprite, &mut Visibility), (With<LPTableCards>, Without<LPHandCards>, Without<RPTableCards>)>,
    mut lp_hand_image_query: Query<(&mut Sprite, &mut Visibility), (With<LPHandCards>, Without<LPTableCards>, Without<RPTableCards>)>,
    mut rp_tablecards_image_query: Query<(&mut Sprite, &mut Visibility), (With<RPTableCards>, Without<LPHandCards>, Without<LPTableCards>)>,
) {
    for player in player_query.iter() {
        if local_players.0.contains(&player.handle) {
            // show hand
            for (i, (mut image_handle, mut vis)) in lp_hand_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().hand[i]);
                *vis = Visibility::Visible;
            }

            // show faceup table cards
            for (i, (mut image_handle, mut vis)) in lp_tablecards_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards[i]);
                *vis = Visibility::Visible;
            }
        }
        else {
            for (i, (mut image_handle, mut vis)) in rp_tablecards_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards[i]);
                *vis = Visibility::Visible;
            }
        }
    }
}

pub fn card_to_asset(
    asset_server: &Res<AssetServer>,
    card: Card,
) -> Sprite {
    let card_num: &str;
    let card_suit: &str;

    match card.number {
        CardVal::Two => card_num = "2.png",
        CardVal::Three => card_num = "3.png",
        CardVal::Four => card_num = "4.png",
        CardVal::Five => card_num = "5.png",
        CardVal::Six => card_num = "6.png",
        CardVal::Seven => card_num = "7.png",
        CardVal::Eight => card_num = "8.png",
        CardVal::Nine => card_num = "9.png",
        CardVal::Ten => card_num = "10.png",
        CardVal::Jack => card_num = "J.png",
        CardVal::Queen => card_num = "Q.png",
        CardVal::King => card_num = "K.png",
        CardVal::Ace => card_num = "A.png",
    }

    match card.suit {
        Suit::Club => card_suit = r"club\cardClubs_",
        Suit::Heart => card_suit = r"heart\cardHearts_",
        Suit::Diamond => card_suit = r"diamond\cardDiamonds_",
        Suit::Spade => card_suit = r"spade\cardSpades_",
    }
    let card_asset = format!("{}{}{}",CARD_LOCATION,card_suit,card_num);
    asset_server.load(card_asset).into()
}

pub fn process_inputs(
    local_players: Res<LocalPlayers>,
    mut player_query: Query<&mut Player>,
    inputs: Res<PlayerInputs<Config>>,
) {
    for player in player_query.iter() {
        if local_players.0.contains(&player.handle) {
            // actions are taken the local player

            // TODO: Numbers represent selected cards
            // TODO: Enter -> check if selected card is possible to play and play them
            // TODO: T -> add cards from pile to players hand
            // TODO: P -> add cards from deck until player has 3 (or deck is empty)
        }
        else {
            // actions are for the remote player
        }
    }
}

