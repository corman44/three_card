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
        components::{ActionType, GameRoom, PlayerCommand},
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
        player.hand = vec![
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
            card_deck.cards.cards.pop().unwrap(),
        ];
        player.hand.sort();
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
            for (i, (mut image_handle, mut vis)) in lp_hand_image_query.iter_mut().enumerate() {
                *image_handle = card_to_asset(&asset_server, player.clone().hand[i]);
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
            .expect("expected button press..").to_digit();
        if !(key + 1 > player_query.iter().find(|p| p.handle == *local_players.0.first().unwrap()).unwrap().hand.len() as u8) {
            if !selected_cards.cards.contains(&key) {
                selected_cards.cards.insert(key);
            } else {
                selected_cards.cards.remove(&key);
            }
            dbg!(&selected_cards);
        } else {
            info!("Selected Card {} is out of range of Hand", key);
        }
    }
}

pub fn play_local_cards(
    local_players: Res<LocalPlayers>,
    player_turn: Res<PlayerTurn>,
    mut selected_cards: ResMut<SelectedCards>,
    mut room: ResMut<GameRoom>,
    button: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Player>,
    mut card_pile: ResMut<Pile>,
) {
    if button.just_pressed(KeyCode::KeyC) {
        let mut player = players
            .iter_mut()
            .find(|p| p.handle == *local_players.0.first().unwrap())
            .expect("no LP found");
        let mut cards_to_play: Vec<Card> = vec![];
        for (count, idx) in selected_cards.cards.clone().iter().enumerate() {
            dbg!(&player.hand);
            dbg!(&format!("count, idx: {}, {}", count, idx));
            cards_to_play.push(player.hand.remove(*idx as usize - count));
        }

        for card in cards_to_play {
            card_pile.cards.push(card.to_num());
        }

        let cmd = PlayerCommand {
            action: ActionType::PlayCards,
            data: Some(selected_cards.cards.clone().iter().copied().collect::<Vec<u8>>()),
        };
        room.send(cmd);
        // TODO Update displayed cards
    }
}

pub fn rx_other_players(
    mut room: ResMut<GameRoom>,
    mut player_turn: ResMut<PlayerTurn>,
    mut players_query: Query<&mut Player>,
) {
    room.socket.update_peers();
    for (id, msg) in room.receive() {}
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

fn play_cards() {}
