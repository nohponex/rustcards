use crate::deck::stack::Stack;

mod deck;

fn main() {
    let s = Stack::empty();
    println!("{}", s.len())
}
