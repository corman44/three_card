use three_card::{self, game::components::{Card, CardVal, Suit}};

#[test]
pub fn card_conversions() {
    let c1 = Card {number: CardVal::Jack, suit: Suit::Diamond};
    let c2 = Card {number: CardVal::Ace, suit: Suit::Club};
    let c3 = Card {number: CardVal::Two, suit: Suit::Heart};
    assert_eq!(c1, Card::from_u8(48));
    assert_eq!(c2, Card::from_u8(12));
    assert_eq!(c3, Card::from_u8(13));

    assert_eq!(c1.to_num(), 48);
    assert_eq!(c2.to_num(), 12);
    assert_eq!(c3.to_num(), 13);
}