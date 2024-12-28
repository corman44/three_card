use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use three_card::{game::{components::Card, CardDeck, CardVal, GamePlugin, Suit}, networking::MyNetworkingPlugin, read_local_inputs, setup, AppState, Config};

/*
    Currently based on Matchbox Guide:
 - https://johanhelsing.studio/posts/extreme-bevy
 - https://github.com/johanhelsing/matchbox

 */

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
            GamePlugin,
            GgrsPlugin::<Config>::default(),
            MyNetworkingPlugin,
    ))
        // .rollback_component_with_clone::<Transform>()
        // .rollback_resource_with_clone::<CardDeck>()
        .init_resource::<CardDeck>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .init_state::<AppState>()
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(ReadInputs, read_local_inputs)
        .run();
}

#[cfg(test)]
mod tests {
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

