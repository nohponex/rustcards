use std::fmt;

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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
    }
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

    pub fn equals(&self, card: &Card) -> bool {
        return self.suit == card.suit && self.rank == card.rank;
    }
    pub fn rank(&self) -> Rank {
        self.rank
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card::{Card, Rank, Suit};
    #[test]
    fn test_equals() {
        assert!(Card::new(Rank::Ace, Suit::Clubs).equals(&Card::new(Rank::Ace, Suit::Clubs)))
    }
    #[test]
    fn test_not_equals() {
        assert!(!Card::new(Rank::Ace, Suit::Clubs).equals(&Card::new(Rank::Two, Suit::Clubs)));

        assert!(!Card::new(Rank::Ace, Suit::Clubs).equals(&Card::new(Rank::Ace, Suit::Hearts)));
    }
    #[test]
    fn test_print() {
        assert_eq!(format!("{}", Card::new(Rank::Ace, Suit::Clubs)), "[A, ♣]");
    }
}
