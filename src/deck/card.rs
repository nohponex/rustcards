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
