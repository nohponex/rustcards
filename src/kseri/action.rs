use crate::deck::card::Card;

pub enum Action {
    Played(Card),
    Folded
}
