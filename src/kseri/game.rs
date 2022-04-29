use crate::deck::stack;
use crate::kseri::player::Player;
use crate::Stack;
use std::collections::HashMap;
use std::thread::current;

struct Game {
    current_player: Player,
    played: Stack,
    deck: Stack,
    number_of_players: u8,
    stacks: HashMap<Player, Stack>,
}
impl Game {
    fn new(number_of_players: u8, mut deck: Stack) -> Game {
        let mut played = Stack::empty();
        //play 4 cards
        for n in 1..5 {
            let c = deck.pop().unwrap();
            played.push(c)
        }
        let mut stacks = HashMap::with_capacity(number_of_players as usize);

        let mut current_player = Player::Player1;

        for n in 1..number_of_players + 1 {
            let mut player_stack = Stack::empty();
            for n in 1..7 {
                let c = deck.pop().unwrap();
                player_stack.push(c);
            }
            stacks.insert(current_player, player_stack);
            current_player = current_player.next(number_of_players)
        }

        Game {
            current_player: current_player,
            number_of_players: number_of_players,
            played: played,
            stacks: stacks,
            deck: deck,
        }
    }
}

mod test {
    use crate::deck::deck::deck;
    use crate::kseri::game::Game;
    use crate::kseri::player::Player;

    #[test]
    fn testNewGame() {
        let g = Game::new(4, deck());

        assert_eq!(g.current_player, Player::Player1);
        assert_eq!(g.number_of_players, 4);
        assert_eq!(g.played.len(), 4);
        assert_eq!(g.deck.len(), 52 - 4 - g.number_of_players as u32 * 6);

        assert_eq!(g.stacks.get(&Player::Player1).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player2).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player3).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player4).unwrap().len(), 6);
    }
}
