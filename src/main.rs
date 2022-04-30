extern crate core;

use crate::deck::deck::deck;
use crate::deck::stack::Stack;
use crate::kseri::action::Action;
use crate::kseri::game::Game;

mod deck;
mod kseri;

fn main() {
    let mut g: Game = Game::new(4, deck());

    while !g.ended() {
        g.print();
        let card_to_play = g.stacks.get(&g.current_player()).unwrap().peek().unwrap();
        g.apply(Action::Played(card_to_play.clone()));
    }
    g.print();
}
