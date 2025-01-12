use bevy::prelude::*;
use three_card::{dev_tools::DevToolsPlugin, game::{components::CardDeck, GamePlugin}, networking::MyNetworkingPlugin, setup, AppState};

/*
    Currently based on Matchbox Guide:
 - https://johanhelsing.studio/posts/extreme-bevy
 - https://github.com/johanhelsing/matchbox

 */

// Main Priorities!!
// TODO how to select cards to play?
// TODO apply game logic before allowing player to play cards
// TODO allow for player to pickup from Deck whenever (if less than 3 in hand)
// TODO utilize faceup cards when out of hand cards and deck is empty
// TODO utilize facedown cards when out of hand cards and out of faceup cards
// TODO show facedown card shorlty after play attempt
// TODO win condition if player is out of Hand Cards, Faceup Cards, Facedown Cards, and Deck Cards

// others
// TODO rework AppState and DeckState to properly utilze each
// TODO highlight selected card(s)
// TODO only display card change if there is a change (Changed<>)
// TODO add player turn notification (Arrow, highlights, or something green)
// TODO Iterate over each type of card to display (optimization)

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
    use three_card::game::components::{Card, CardVal, Suit};
    
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

