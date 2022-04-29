use std::fmt;

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

mod test {
    use crate::kseri::player::Player;

    #[test]
    fn test_next() {
        assert_eq!(Player::Player1.next(2), Player::Player2);

        assert_eq!(Player::Player3.next(3), Player::Player1);
        assert_eq!(Player::Player2.next(3), Player::Player3);

        assert_eq!(Player::Player3.next(4), Player::Player4);
    }
}
