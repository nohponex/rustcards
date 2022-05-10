use crate::deck::card::{Card, Rank, Suit};
use crate::Stack;

impl Card {
    pub fn points(&self) -> u32 {
        match (self.rank(), self.suit()) {
            (Rank::Ace | Rank::King | Rank::Queen | Rank::Jack, _) => 1,
            (Rank::Two, Suit::Clubs) => 1,
            (Rank::Ten, Suit::Diamonds) => 2,
            (_, _) => 0,
        }
    }
}

impl Stack {
    pub fn points(&self) -> u32 {
        self
            .iter()
            .map(Card::points).sum()
    }
}
mod test {
    use crate::deck::card::{Card, Rank, Suit};
    use crate::Stack;

    #[test]
    fn test(){
        let s = Stack::from_vec(vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
        ]);

        assert_eq!(s.points(), 4);
    }
}