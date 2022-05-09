use crate::deck::card::{Card, Rank};
use crate::deck::stack;
use crate::kseri::action::Action;
use crate::kseri::player::{GameOfPlayers, Player};
use crate::kseri::points;
use crate::Stack;
use std::collections::HashMap;
use std::iter::Iterator;

pub(crate) struct Game {
    current_player: Player,
    played: Stack,
    deck: Stack,
    number_of_players: u8,
    pub(crate) stacks: HashMap<Player, Stack>,
    picked: HashMap<Player, Stack>,
    points: HashMap<Player, u32>,
    ended: bool,
}
impl Game {
    pub(crate) fn new(number_of_players: u8, mut deck: Stack) -> Game {
        let mut played = Stack::empty();
        //play 4 cards
        for n in 1..5 {
            let c = deck.pop().unwrap();
            played.push(c)
        }

        let players_iter = || GameOfPlayers::new(number_of_players);

        let stacks = players_iter().map(|key| (key, Stack::empty())).collect();
        let picked = players_iter().map(|key| (key, Stack::empty())).collect();
        let points = players_iter().map(|key| (key, 0)).collect();

        let mut g = Game {
            current_player: Player::Player1,
            number_of_players: number_of_players,
            played: played,
            stacks: stacks,
            deck: deck,
            picked: picked,
            points: points,
            ended: false,
        };

        g.deal_cards();

        return g;
    }

    fn deal_cards(&mut self) {
        for p in self.players_iter() {
            let player_stack = self.stacks.get_mut(&p).unwrap();
            for _ in 1..7 {
                let c = self.deck.pop().unwrap();
                player_stack.push(c);
            }
        }
    }

    //todo return iterator signature
    fn players_iter(&self) -> GameOfPlayers {
        GameOfPlayers::new(self.number_of_players)
    }

    pub(crate) fn apply(&mut self, action: Action) {
        match action {
            Action::Played(card) => {
                //make sure player had this card and remove it
                if !self
                    .stacks
                    .get_mut(&self.current_player)
                    .unwrap()
                    .remove(&card)
                {
                    panic!("not allowed card");
                }

                println!("Player {} played {}", self.current_player, card);

                match (self.played.peek(), card.rank()) {
                    (Some(a), b) if a.rank() == b => {
                        if self.played.len() == 1 {
                            self.points.entry(self.current_player).and_modify(|v| {
                                *v += match card.rank() {
                                    Rank::Jack => 20,
                                    _ => 10,
                                }
                            });
                            println!("Kseri!")
                        }
                        self.points
                            .entry(self.current_player)
                            .and_modify(|v| *v += self.played.points() + card.points());

                        let picked = self.picked.get_mut(&self.current_player).unwrap();
                        for c in self.played.iter() {
                            picked.push(*c);
                        }

                        self.played = Stack::empty();
                        picked.push(card);
                    }
                    (Some(_), Rank::Jack) => {
                        self.points
                            .entry(self.current_player)
                            .and_modify(|v| *v += self.played.points() + card.points());

                        let picked = self.picked.get_mut(&self.current_player).unwrap();
                        for c in self.played.iter() {
                            picked.push(*c);
                        }
                        self.played = Stack::empty();
                        picked.push(card);
                    }
                    (_, _) => self.played.push(card),
                }

                self.current_player = self.current_player.next(self.number_of_players);

                if self.should_deal() {
                    self.deal_cards();
                }

                if self.was_last_card_played() {
                    let player_with_most_cards = self.player_with_most_cards();
                    println!("most cards: Player {}", player_with_most_cards);
                    self.points
                        .entry(*player_with_most_cards)
                        .and_modify(|v| *v += 3);

                    self.ended = true;
                }
            }
        }
    }

    fn should_deal(&self) -> bool {
        return !self.was_last_card_played()
            && self.current_player == Player::Player1
            && self.stacks.get(&Player::Player1).unwrap().len() == 0;
    }

    fn was_last_card_played(&self) -> bool {
        return self.stacks.iter().map(|(_, s)| s.len()).max() == Some(0) && self.deck.len() == 0;
    }

    pub fn print(&self) {
        if self.ended {
            for p in self.players_iter() {
                println!("Player {} has {} points", p, self.points.get(&p).unwrap())
            }
            println!("game over! Winner is Player {}!", self.winner().unwrap());

            return;
        }
        println!();
        println!("cards down: {}", self.played);
        println!("current player turn: {}", self.current_player);
        println!(
            "player can play on of: {}",
            self.stacks.get(&self.current_player).unwrap()
        );
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    fn winner(&self) -> Option<&Player> {
        match self.ended {
            true => Some(self.points.iter().max_by_key(|&(_, v)| v).unwrap().0),
            false => None,
        }
    }

    fn player_with_most_cards(&self) -> &Player {
        return self
            .picked
            .iter()
            .map(|(p, s)| (p, s.len()))
            .max_by_key(|&(p, l)| l)
            .unwrap()
            .0;
    }
}

mod test {
    use crate::deck::card::{Card, Rank, Suit};
    use crate::deck::deck::deck;
    use crate::kseri::action::Action;
    use crate::kseri::game::Game;
    use crate::kseri::player::Player;
    use crate::Stack;

    #[test]
    fn testNewGame() {
        let mut g = Game::new(4, deck());

        assert_eq!(g.current_player, Player::Player1);
        assert_eq!(g.number_of_players, 4);
        assert_eq!(g.played.len(), 4);
        assert_eq!(g.deck.len(), 52 - 4 - g.number_of_players as u32 * 6);

        assert_eq!(g.stacks.get(&Player::Player1).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player2).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player3).unwrap().len(), 6);
        assert_eq!(g.stacks.get(&Player::Player4).unwrap().len(), 6);
    }

    #[test]
    fn test_give_cards() {
        let mut g = Game::new(4, deck());

        g.deal_cards();

        assert_eq!(g.deck.len(), 52 - 4 - (4 * 12));
        assert_eq!(g.stacks.get(&Player::Player1).unwrap().len(), 12);
        assert_eq!(g.stacks.get(&Player::Player2).unwrap().len(), 12);
        assert_eq!(g.stacks.get(&Player::Player3).unwrap().len(), 12);
        assert_eq!(g.stacks.get(&Player::Player4).unwrap().len(), 12);
    }

    #[test]
    fn testPrint() {
        let g = Game::new(4, deck());
        g.print()
    }

    #[test]
    fn test_apply() {
        let mut g = Game::new(
            2,
            Stack::from_vec(vec![
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Four, Suit::Spades),
                //p1
                Card::new(Rank::Five, Suit::Clubs),
                Card::new(Rank::Six, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Clubs),
                Card::new(Rank::Eight, Suit::Clubs),
                Card::new(Rank::Nine, Suit::Clubs),
                Card::new(Rank::Jack, Suit::Clubs),
                //p2
                Card::new(Rank::Five, Suit::Diamonds),
                Card::new(Rank::Six, Suit::Diamonds),
                Card::new(Rank::Seven, Suit::Diamonds),
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Nine, Suit::Diamonds),
                Card::new(Rank::Ten, Suit::Diamonds),
            ]),
        );
        assert_eq!(*g.points.get(&Player::Player1).unwrap(), 0);
        assert_eq!(g.played.len(), 4);
        assert_eq!(g.picked.get(&Player::Player1).unwrap().len(), 0);

        g.apply(Action::Played(Card::new(Rank::Jack, Suit::Clubs)));
        assert_eq!(*g.points.get(&Player::Player1).unwrap(), 20);
        assert_eq!(g.played.len(), 0);
        assert_eq!(g.picked.get(&Player::Player1).unwrap().len(), 5);
    }

    #[test]
    fn test_game() {
        let mut g: Game = Game::new(4, deck());

        while !g.ended {
            g.print();
            let card_to_play = g.stacks.get(&g.current_player).unwrap().peek().unwrap();
            g.apply(Action::Played(card_to_play.clone()));
        }
        g.print();
    }
}
