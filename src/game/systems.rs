use std::collections::BTreeSet;

use bevy::prelude::*;

use super::{
    components::{
        Card, CardVal, Deck, KeyToDigit, LPHandCards, LPTableCards, LocalPlayers, Pile, Player,
        PlayerTurnState, PlayerTurnText, RPHandCards, RPTableCards, SelectedCards, ShortWait, Suit,
    },
    CardDeck, DeckState, PlayerTurn,
};
use crate::{
    networking::{
        components::{ActionType, GameRoom, IntoU64, PlayerCommand},
        SessionSeed,
    },
    AppState,
};

pub const CARD_LOCATION: &str = r"normal_cards\individual\";
pub const CARD_BACK_LOACTION: &str = r"normal_cards\individual\card back\cardBackGreen.png";
pub const CARD_SCALE: f32 = 0.15;

pub fn setup(
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
                translation: Vec3::new(-100. + 100. * idx as f32, -180., 1.),
                scale: Vec3::new(CARD_SCALE + 0.05, CARD_SCALE + 0.05, 1.),
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
                translation: Vec3::new(-400. + 75. * idx as f32, -190., 1.),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..default()
            },
            LPTableCards(idx),
        ));

        // RP Faceup
        commands.spawn((
            Sprite {
                image: asset_server.load(r"normal_cards\individual\heart\cardHearts_2.png"),
                ..default()
            },
            Visibility::Hidden,
            Transform {
                translation: Vec3::new(175. + 75. * idx as f32, 190., 1.),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..default()
            },
            RPTableCards(idx),
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
        Deck,
    ));

    // Spawn Player Turn Text
    commands.spawn((
        PlayerTurnText,
        Text2d::new(""),
        TextFont {
            font_size: 20.,
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(20., 250., 1.),
            ..default()
        },
        Visibility::Hidden,
    ));

    // next_app_state.set(AppState::GameStart);
}

pub fn deal_cards(
    mut commands: Commands,
    mut players: Query<&mut Player>,
    mut card_deck: ResMut<CardDeck>,
    mut next_deck_state: ResMut<NextState<DeckState>>,
    sesh_seed: Res<SessionSeed>,
) {
    // first shuffle the deck
    card_deck.shuffle(**sesh_seed);
    next_deck_state.set(DeckState::Shuffled);

    // for each player in the game, deal 3 cards facedown, 3 faceup, 3 to the hand
    for mut player in players.iter_mut() {
        player.facedown_cards = vec![
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
        ];
        player.faceup_cards = vec![
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
        ];
        player.hand.insert(card_deck.cards.cards.pop().unwrap());
        player.hand.insert(card_deck.cards.cards.pop().unwrap());
        player.hand.insert(card_deck.cards.cards.pop().unwrap());
    }

    // Workaround for not have LocalPlayer created yet...
    commands.insert_resource(ShortWait {
        timer: Timer::from_seconds(1., TimerMode::Once),
    });

    next_deck_state.set(DeckState::Dealt);
}

pub fn short_wait(
    time: Res<Time>,
    mut short_wait: ResMut<ShortWait>,
    mut next_deck_state: ResMut<NextState<DeckState>>,
) {
    if short_wait.timer.tick(time.delta()).just_finished() {
        next_deck_state.set(DeckState::Dealt);
    } else {
        // info!("ShortWait: {:?}", short_wait.timer.fraction() * 100.);
    }
}

pub fn display_table_cards(
    asset_server: Res<AssetServer>,
    local_players: Res<LocalPlayers>,
    player_query: Query<&Player>,
    mut next_deck_state: ResMut<NextState<DeckState>>,
    mut lp_hand_image_query: Query<
        (&mut Sprite, &mut Visibility),
        (With<LPHandCards>, Without<LPTableCards>),
    >,
    mut lp_tablecards_image_query: Query<(&mut Sprite, &mut Visibility), With<LPTableCards>>,
    mut rp_tablecards_image_query: Query<
        (&mut Sprite, &mut Visibility),
        (
            With<RPTableCards>,
            Without<LPHandCards>,
            Without<LPTableCards>,
        ),
    >,
) {
    dbg!(&local_players.0);
    dbg!(&player_query.iter().map(|a| a.handle).collect::<Vec<_>>());
    for player in player_query.iter() {
        if local_players.0.contains(&(player.handle as u64)) {
            // show hand
            for ((mut image_handle, mut vis), card) in
                lp_hand_image_query.iter_mut().zip(player.hand.clone())
            {
                *image_handle = card_to_asset(&asset_server, card);
                *vis = Visibility::Visible;
            }

            // show faceup table cards
            for (i, (mut image_handle, mut vis)) in lp_tablecards_image_query.iter_mut().enumerate()
            {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards[i]);
                *vis = Visibility::Visible;
            }
        } else {
            for (i, (mut image_handle, mut vis)) in rp_tablecards_image_query.iter_mut().enumerate()
            {
                *image_handle = card_to_asset(&asset_server, player.clone().faceup_cards[i]);
                *vis = Visibility::Visible;
            }
        }
    }
    next_deck_state.set(DeckState::Gameplay)
}

pub fn display_turn(
    local_players: Res<LocalPlayers>,
    player_turn: Res<PlayerTurn>,
    mut turn_text_query: Query<
        (&mut Visibility, &mut Text2d, &mut TextColor),
        With<PlayerTurnText>,
    >,
    // mut next_deck_state: ResMut<NextState<DeckState>>,
) {
    if local_players
        .0
        .contains(&(player_turn.ids[player_turn.turn] as u64))
    {
        let (mut vis, mut txt, mut color) = turn_text_query.single_mut();
        *vis = Visibility::Visible;
        txt.0 = String::from("YOUR TURN");
        color.0 = Color::srgb(0.294, 0.969, 0.337);
    } else {
        let (mut vis, mut txt, mut color) = turn_text_query.single_mut();
        *vis = Visibility::Visible;
        txt.0 = String::from("OTHER PLAYERS TURN..");
        color.0 = Color::srgb(0.9, 0.9, 0.0);
    }

    // next_deck_state.set(DeckState::Gameplay);
}

pub fn card_to_asset(asset_server: &Res<AssetServer>, card: Card) -> Sprite {
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
    let card_asset = format!("{}{}{}", CARD_LOCATION, card_suit, card_num);
    asset_server.load(card_asset).into()
}

pub fn select_cards(
    local_players: Res<LocalPlayers>,
    player_turn: Res<PlayerTurn>,
    player_query: Query<&Player>,
    mut selected_cards: ResMut<SelectedCards>,
    button: Res<ButtonInput<KeyCode>>,
) {
    if button.any_just_pressed([
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
        KeyCode::Digit0,
    ]) {
        let key = button
            .get_just_pressed()
            .next()
            .expect("expected button press..")
            .to_digit();
        let lp = player_query
            .iter()
            .find(|p| p.handle == *local_players.0.first().unwrap())
            .unwrap();
        // check for player turn and card selected is in range
        if !(key + 1 > lp.hand.len() as u8) && player_turn.current_turn() == lp.handle {
            match lp.hand.iter().collect::<Vec<_>>().get(key as usize) {
                Some(card) => {
                    if selected_cards.cards.contains(*card) {
                        selected_cards.cards.remove(*card);
                        dbg!(&selected_cards);
                    } else if selected_cards.cards.is_empty() {
                        selected_cards.cards.insert(**card);
                        dbg!(&selected_cards);
                    } else if card.number
                        == selected_cards
                            .cards
                            .iter()
                            .collect::<Vec<Card>>()
                            .first()
                            .expect("expected card in selcted cards")
                            .number
                    {
                        selected_cards.cards.insert(**card);
                        dbg!(&selected_cards);
                    } else {
                        info!(
                            "Selecting Card: {:?} not possible due to already Selected Card: {:?}",
                            *card, selected_cards.cards,
                        );
                    }
                }
                None => {
                    dbg!(&format!("Error accessing lp.hand.get({})", key));
                }
            }
            // dbg!(&selected_cards);
        } else if player_turn.current_turn() != *local_players.0.first().unwrap() {
            info!("Not your turn...");
        } else {
            info!("Selected Card {} is out of range of Hand", key);
        }
    }
}

pub fn play_cards(
    card_pile: &mut Pile,
    player: &mut Player,
    player_turn: &mut PlayerTurn,
    room: Option<&mut GameRoom>,
    selected_cards: &mut Vec<Card>,
) {
    // check if sending or rx
    if let Some(room) = room {
        let cmd = PlayerCommand {
            action: ActionType::PlayCards,
            data: Some(
                selected_cards
                    .clone()
                    .iter()
                    .copied()
                    .map(|c| c.to_num())
                    .collect::<Vec<u8>>(),
            ),
        };
        room.send(cmd);
    }
    // play cards
    for card in selected_cards.clone() {
        card_pile.cards.push(card);
    }
    for card in selected_cards {
        player.hand.remove(&card);
    }
    selected_cards.clear();
    player_turn.next();
}

pub fn rx_other_players(
    mut card_deck: ResMut<CardDeck>,
    mut card_pile: ResMut<Pile>,
    mut player_turn: ResMut<PlayerTurn>,
    mut players_query: Query<&mut Player>,
    mut room: ResMut<GameRoom>,
    mut selected_cards: ResMut<SelectedCards>,
) {
    room.socket.update_peers();
    for (id, msg) in room.receive() {
        match msg.action {
            ActionType::PickupPile => {
                let mut player = players_query
                    .iter_mut()
                    .find(|p| p.handle == id.into_u64())
                    .expect("unable to find player");
                pickup_pile(
                    &mut card_pile,
                    &mut player,
                    &mut player_turn,
                    None,
                    &mut selected_cards,
                );
                player_turn.next();
            }
            ActionType::PickupDeck => {
                let mut player = players_query
                    .iter_mut()
                    .find(|p| p.handle == id.into_u64())
                    .expect("unable to find player");
                pickup_deck(&mut card_deck, &mut player, None, &mut selected_cards);
            }
            ActionType::PlayCards => {
                dbg!(&format!("Player {} playing cards: 
                {:?}", id.into_u64(), msg.data.unwrap()));
                let mut player = players_query
                    .iter_mut()
                    .find(|p| p.handle == id.into_u64())
                    .expect("unable to find player");
                play_cards(
                    &mut card_pile,
                    &mut player,
                    &mut player_turn,
                    None,
                    &mut selected_cards
                );
            }
        }
    }
}

pub fn update_player_turn_state(
    local_players: Res<LocalPlayers>,
    player_turn: Res<PlayerTurn>,
    mut player_turn_state: ResMut<NextState<PlayerTurnState>>,
) {
    if player_turn.is_changed() {
        if *local_players.0.first().unwrap() == player_turn.current_turn() {
            player_turn_state.set(PlayerTurnState::LocalPlayerTurn);
        } else {
            player_turn_state.set(PlayerTurnState::RemotPlayerTurn(player_turn.current_turn()));
        }
    }
}

pub fn pickup_pile(
    card_pile: &mut Pile,
    player: &mut Player,
    player_turn: &mut PlayerTurn,
    room: Option<&mut GameRoom>,
    selected_cards: &mut SelectedCards,
) {
    for card in card_pile.cards.clone() {
        player.hand.insert(card);
    }
    selected_cards.cards.clear();
    card_pile.cards.clear();
    player_turn.next();

    if let Some(room) = room {
        room.send(PlayerCommand {
            action: ActionType::PickupPile,
            data: None,
        });
    }
}

pub fn monitor_lp_inputs(
    button: Res<ButtonInput<KeyCode>>,
    mut card_pile: ResMut<Pile>,
    mut card_deck: ResMut<CardDeck>,
    local_players: Res<LocalPlayers>,
    mut players: Query<&mut Player>,
    mut player_turn: ResMut<PlayerTurn>,
    mut room: ResMut<GameRoom>,
    mut selected_cards: ResMut<SelectedCards>,
) {
    // Pile Pickup
    if player_turn.current_turn() == *local_players.0.first().unwrap()
        && button.just_pressed(KeyCode::KeyP)
    {
        pickup_pile(
            &mut card_pile,
            &mut players
                .iter_mut()
                .find(|p| p.handle == *local_players.0.first().unwrap())
                .expect("no player found"),
            &mut player_turn,
            Some(&mut room),
            &mut selected_cards,
        )
    }

    // Deck Pickup
    if button.just_pressed(KeyCode::KeyD) && card_deck.cards.cards.len() > 0 {
        let mut player = players
            .iter_mut()
            .find(|p| p.handle == *local_players.0.first().unwrap())
            .expect("no player found");
        if player.hand.len() < 3 {
            pickup_deck(&mut card_deck, &mut player, Some(&mut room), &mut selected_cards)
        }
    }

    // Card Play
    if player_turn.current_turn() == *local_players.0.first().unwrap()
        && button.just_pressed(KeyCode::KeyC)
        && selected_cards.cards.len() > 0
    {
        let mut player = players
            .iter_mut()
            .find(|p| p.handle == *local_players.0.first().unwrap())
            .expect("no player found");

        if card_pile.cards.is_empty()
            || selected_cards.cards.first().unwrap().number
                >= card_pile.cards.last().unwrap().number
        {
            play_cards(
                &mut card_pile,
                &mut player,
                &mut player_turn,
                Some(&mut room),
                &mut selected_cards,
            );
        }
    }
}

pub fn pickup_deck(
    deck: &mut CardDeck,
    player: &mut Player,
    room: Option<&mut GameRoom>,
    selected_cards: &mut SelectedCards,
) {
    for idx in 0..(3 - player.hand.len()) {
        player
            .hand
            .insert(deck.cards.cards.pop().expect("no cards left in deck"));
    }

    selected_cards.cards.clear();
    if let Some(room) = room {
        room.send(PlayerCommand {
            action: ActionType::PickupDeck,
            data: None,
        });
    }
}
