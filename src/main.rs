use bevy::prelude::*;
use three_card::{dev_tools::DevToolsPlugin, game::{components::{Card, CardDeck}, GamePlugin}, networking::MyNetworkingPlugin, setup, AppState};

/*
    Currently based on Matchbox Guide:
 - https://johanhelsing.studio/posts/extreme-bevy
 - https://github.com/johanhelsing/matchbox

 */

// Main Priorities!!
// TODO Properly Integrate PlayerTurn, Peer ID, and Player together for connection and card display/placement
// TODO send player turn update from each client to each other (only if it's your turn can you request the next turn)

// others
// TODO: highlight selected card(s)
// TODO: only display card change if there is a change (Changed<>)
// TODO: add player turn notification (Arrow, highlights, or something green)
// TODO: Iterate over each type of card to display (optimization)

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            DevToolsPlugin,
            GamePlugin,
            MyNetworkingPlugin,
        ))
        .init_resource::<CardDeck>()
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (
            setup,
        ))
        // .add_systems(ReadInputs, read_local_inputs)
        .run();
}

#[cfg(test)]
mod tests {
    use three_card::game::components::{CardVal, Suit};

    use super::*;

    #[test]
    pub fn test_ordering() {
        let mut cards = vec![
            Card {
                number: CardVal::Ace,
                suit: Suit::Club,
            },
            Card {
                number: CardVal::Jack,
                suit: Suit::Spade,
            },
            Card {
                number: CardVal::Four,
                suit: Suit::Heart,
            },
            Card {
                number: CardVal::Three,
                suit: Suit::Diamond,
            },
            Card {
                number: CardVal::Six,
                suit: Suit::Diamond,
            },
            Card {
                number: CardVal::Ace,
                suit: Suit::Spade,
            },
        ];
        cards.sort();
        dbg!(&cards);
        assert_eq!(43, cards.iter().last().unwrap().to_num());
    }
}

