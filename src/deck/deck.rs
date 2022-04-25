use crate::deck::card::{Card, Rank, Suit};
use crate::Stack;

pub fn Deck() -> Stack {
    Stack::fromVec(vec![
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Three, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Six, Suit::Clubs),

        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::Two, Suit::Diamonds),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Five, Suit::Diamonds),
        Card::new(Rank::Six, Suit::Diamonds),

        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Six, Suit::Hearts),

        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Two, Suit::Spades),
        Card::new(Rank::Three, Suit::Spades),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Five, Suit::Spades),
        Card::new(Rank::Six, Suit::Spades),
    ])
}


#[cfg(test)]
mod tests {
    use crate::deck::deck::Deck;

    #[test]
    fn test_default_deck_should_have_52_cards() {
        let s = Deck();
        assert_eq!(s.len(), 52);
    }
}
