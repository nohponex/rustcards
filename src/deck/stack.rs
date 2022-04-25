use crate::deck::card::Card;

pub struct Stack {
    cards: Vec<Card>,
}

impl Stack {
    pub fn newEmpty() -> Stack {
        Stack { cards: vec![] }
    }

    pub fn fromVec(from: Vec<Card>) -> Stack {
        Stack { cards: from }
    }

    fn push(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn peek(&self) -> Option<&Card> {
        self.cards.get(0)
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn len(&self) -> u32 {
        self.cards.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card::{Card, Rank, Suit};
    use crate::deck::stack::Stack;

    #[test]
    fn test_peek_given_empty() {
        let s = Stack::newEmpty();
        assert_eq!(s.len(), 0);

        assert!(s.peek().is_none())
    }

    #[test]
    fn test_peek_given_not_empty() {
        let card = Card::new(Rank::Ace, Suit::Clubs);

        let mut s = Stack::newEmpty();
        s.push(card);
        assert_eq!(s.len(), 1);

        let peeked = s.peek().unwrap();

        assert!(peeked.equals(Card::new(Rank::Ace, Suit::Clubs)));
    }
}
