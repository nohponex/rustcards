#[derive(PartialEq, Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
#[derive(PartialEq, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn equals(&self, card: Card) -> bool {
        return self.suit == card.suit && self.rank == card.rank;
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card::{Card, Rank, Suit};

    #[test]
    fn test_equals() {
        assert!(Card::new(Rank::Ace, Suit::Clubs).equals(Card::new(Rank::Ace, Suit::Clubs)))
    }
    #[test]
    fn test_not_equals() {
        assert!(!Card::new(Rank::Ace, Suit::Clubs).equals(Card::new(Rank::Two, Suit::Clubs)));

        assert!(!Card::new(Rank::Ace, Suit::Clubs).equals(Card::new(Rank::Ace, Suit::Hearts)));
    }
}
