use std::fmt;
use std::thread::current;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Player {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Player::Player1 => write!(f, "1"),
            Player::Player2 => write!(f, "2"),
            Player::Player3 => write!(f, "3"),
            Player::Player4 => write!(f, "4"),
        }
    }
}

impl Player {
    pub fn next(&self, number_of_players: u8) -> Player {
        assert!(number_of_players > 0);
        assert!(number_of_players <= 4);

        if *self == Player::Player1 {
            return Player::Player2
        }

        if *self == Player::Player2 && number_of_players > 2 {
            return Player::Player3
        }

        if *self == Player::Player3 && number_of_players > 3 {
            return Player::Player4
        }

        return Player::Player1
    }
}

pub struct GameOfPlayers {
    number_of_players: u8,
    current: Player,
    reset: bool,
}

impl GameOfPlayers {
    pub fn new(number_of_players: u8) -> GameOfPlayers {
        GameOfPlayers{
            current: Player::Player1,
            number_of_players,
            reset: false,
        }
    }
}

impl std::iter::Iterator for GameOfPlayers {
    type Item = (Player);

    fn next(&mut self) -> Option<Player> {
        if self.reset && self.current == Player::Player1 {
            return None
        }
        let to_return = self.current.clone();
        self.current = self.current.next(self.number_of_players);

        if self.current == Player::Player1 {
            self.reset = true
        }

        return Some(to_return)
    }
}

mod test {
    use crate::kseri::player::{GameOfPlayers, Player};

    #[test]
    fn test_GameOfPlayers() {
        let mut game = GameOfPlayers::new(4);

        assert_eq!(game.next(), Some(Player::Player1));
        assert_eq!(game.next(), Some(Player::Player2));
        assert_eq!(game.next(), Some(Player::Player3));
        assert_eq!(game.next(), Some(Player::Player4));
        assert_eq!(game.next(), None)
    }

    #[test]
    fn test_next() {
        assert_eq!(Player::Player1.next(2), Player::Player2);

        assert_eq!(Player::Player3.next(3), Player::Player1);
        assert_eq!(Player::Player2.next(3), Player::Player3);

        assert_eq!(Player::Player3.next(4), Player::Player4);
    }
}
