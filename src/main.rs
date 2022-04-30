extern crate core;

use crate::deck::stack::Stack;

mod deck;
mod kseri;

fn main() {
    let s = Stack::empty();
    println!("{}", s.len())
}
