use std::fmt;
use std::slice::Iter;
use crate::deck::card::Card;

use rand::thread_rng;
use rand::seq::SliceRandom;


pub struct Stack {
    cards: Vec<Card>,
}


impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c|  format!("{}", c)).collect::<Vec<_>>().join(", "))
    }
}
impl Stack {
    pub fn empty() -> Stack {
        Stack { cards: vec![] }
    }

    pub fn from_vec(from: Vec<Card>) -> Stack {
        Stack { cards: from }
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn peek(&self) -> Option<Card> {
        match self.cards.get(0)  {
            None => None,
            Some(&x) => Some(x)
        }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn iter(&mut self) -> Iter<'_, Card> {
        self.cards.iter()
    }

    pub fn remove(&mut self, card: &Card) -> bool {
        let len_before = self.len();
        self.cards.retain(|&c| !c.equals(card));

        if self.len() < len_before {
            return true
        }
        return false
    }
    pub fn len(&self) -> u32 {
        self.cards.len() as u32
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card::{Card, Rank, Suit};
    use crate::deck::deck;
    use crate::deck::stack::Stack;

    #[test]
    fn test_peek_given_empty() {
        let s = Stack::empty();
        assert_eq!(s.len(), 0);

        assert!(s.peek().is_none())
    }

    #[test]
    fn test_peek_given_not_empty() {
        let card = Card::new(Rank::Ace, Suit::Clubs);

        let mut s = Stack::empty();
        s.push(card);
        assert_eq!(s.len(), 1);

        let peeked = s.peek().unwrap();

        assert!(peeked.equals(&Card::new(Rank::Ace, Suit::Clubs)));
    }

    #[test]
    fn test_shuffle() {
        let mut stack = deck::deck();

        let first = stack.peek().unwrap();

        stack.shuffle();

        assert!(!stack.peek().unwrap().equals(&first));
    }

    #[test]
    fn test_remove() {
        let mut s = Stack::from_vec(vec![Card::new(Rank::Ace, Suit::Clubs)]);

        assert!(!s.remove(&Card::new(Rank::Two, Suit::Diamonds)));
        assert!(s.remove(&Card::new(Rank::Ace, Suit::Clubs)));

        assert_eq!(s.len(), 0)
    }
}
