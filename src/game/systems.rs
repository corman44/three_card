use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ggrs::{LocalPlayers, PlayerInputs};

use crate::{networking::SessionSeed, AppState, Config};

use super::{components::{Deck, LPHandCards, LPTableCards, Player, RPHandCards, RPTableCards}, Card, CardDeck, CardVal, DeckState, Suit};

pub const CARD_LOCATION: &str = r"normal_cards\individual\";
pub const CARD_BACK_LOACTION: &str = r"normal_cards\individual\card back\cardBackGreen.png";

pub fn setup_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {

    // spawn player1
    commands.spawn((
        NodeBundle::default(),
        Player {
            handle: 0,
            ..default()
        },
    ));

    
    // spawn player2
    commands.spawn((
        NodeBundle::default(),
        Player {
            handle: 1,
            ..default()
        },
    ));

    // Spawn LP Card1
    commands.spawn((
        LPHandCards(0),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
            transform: Transform {
                translation: Vec3::new(-15., -35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // Spawn LP Card2
    commands.spawn((
        LPHandCards(1),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\spade\cardSpades_10.png"),
            transform: Transform {
                translation: Vec3::new(0., -35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // Spawn LP Card3
    commands.spawn((
        LPHandCards(2),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(15., -35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // LocalPlayer Faceup Card1
    commands.spawn((
        LPTableCards(0),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-80., -35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // LocalPlayer Faceup Card2
    commands.spawn((
        LPTableCards(1),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-70., -35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // LocalPlayer Faceup Card3
    commands.spawn((
        LPTableCards(2),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-60., -35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));


    // RP Faceup Card1
    commands.spawn((
        RPTableCards(0),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-80., 35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                rotation: Quat::from_rotation_z(PI),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // RP Faceup Card2
    commands.spawn((
        RPTableCards(1),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-70., 35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                rotation: Quat::from_rotation_z(PI),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // RP Faceup Card3
    commands.spawn((
        RPTableCards(2),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\club\cardClubs_2.png"),
            transform: Transform {
                translation: Vec3::new(-60., 35., 1.),
                scale: Vec3::new(0.022, 0.022, 1.),
                rotation: Quat::from_rotation_z(PI),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // Spawn RP Card1
    commands.spawn((
        RPHandCards(0),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\card back\cardBackGreen.png"),
            transform: Transform {
                translation: Vec3::new(-15., 35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                rotation: Quat::from_rotation_x(PI/8.),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        }
    ));

    // Spawn RP Card2
    commands.spawn((
        RPHandCards(1),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\card back\cardBackGreen.png"),
            transform: Transform {
                translation: Vec3::new(0., 35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                rotation: Quat::from_rotation_x(PI/7.),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        }
    ));

    // Spawn RP Card3
    commands.spawn((
        RPHandCards(2),
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\card back\cardBackGreen.png"),
            transform: Transform {
                translation: Vec3::new(15., 35., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                rotation: Quat::from_rotation_x(PI/6.),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        }
    ));

    // Spawn Deck Card
    commands.spawn((
        Deck,
        SpriteBundle {
            texture: asset_server.load(r"normal_cards\individual\card back\cardBackGreen.png"),
            transform: Transform {
                translation: Vec3::new(5., 0., 1.),
                scale: Vec3::new(0.03, 0.03, 1.),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        }
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
        player.facedown_cards = Some(vec![card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap()]);
        player.faceup_cards = Some(vec![card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap()]);
        player.hand = Some(vec![card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap(), card_deck.cards.pop().unwrap()]);
    }
    // info!("Cards Dealt: \n{:?}", players.iter().collect::<Vec<&Player>>());
    next_deck_state.set(DeckState::Shuffled);
}

pub fn display_table_cards(
    asset_server: Res<AssetServer>,
    local_players: Res<LocalPlayers>,
    player_query: Query<&Player>,
    mut lp_tablecards_image_query: Query<(&mut Handle<Image>, &mut Visibility), (With<LPTableCards>, Without<LPHandCards>, Without<RPTableCards>)>,
    mut lp_hand_image_query: Query<(&mut Handle<Image>, &mut Visibility), (With<LPHandCards>, Without<LPTableCards>, Without<RPTableCards>)>,
    mut rp_tablecards_image_query: Query<(&mut Handle<Image>, &mut Visibility), (With<RPTableCards>, Without<LPHandCards>, Without<LPTableCards>)>,
) {
    for player in player_query.iter() {
        if local_players.0.contains(&player.handle) {
            // show hand
            for (i, (mut image_handle, mut vis)) in lp_hand_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().hand.unwrap()[i]);
                *vis = Visibility::Visible;
            }

            // show faceup table cards
            for (i, (mut image_handle, mut vis)) in lp_tablecards_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards.unwrap()[i]);
                *vis = Visibility::Visible;
            }
        }
        else {
            for (i, (mut image_handle, mut vis)) in rp_tablecards_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards.unwrap()[i]);
                *vis = Visibility::Visible;
            }
        }
    }
}

pub fn card_to_asset(
    asset_server: &Res<AssetServer>,
    card: Card,
) -> Handle<Image> {
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
    // info!("{card_asset}");
    asset_server.load(card_asset)
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

