use crate::deck::stack::Stack;

mod deck;

fn main() {
    let s = Stack::newEmpty();
    println!("{}", s.len())
}
