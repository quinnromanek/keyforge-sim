use turn::{Turn, Action};

use board::DeckBoard;
use rand::thread_rng;
use game::House;

pub trait Player {
    fn choose_house<'a>(&self, mine: &'a DeckBoard, opponent: &'a DeckBoard) -> House;
    fn next_action<'a, 'b>(&self, turn: &'b Turn<'a>) -> Option<Action<'a>>;
}

pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn choose_house<'a>(&self, mine: &'a DeckBoard<'a>, opponent: &'a DeckBoard<'a>) -> House {
        unimplemented!()
    }

    fn next_action<'a, 'b>(&self, turn: &'b Turn<'a>) -> Option<Action<'a>> {
        unimplemented!()
    }
}

pub mod test {
    use player::Player;
    use board::DeckBoard;
    use turn::Turn;
    use turn::Action;
    use game::House;

    pub struct TestPlayer {
        pub house: House,
    }

    impl Player for TestPlayer {
        fn choose_house<'a>(&self, mine: &'a DeckBoard<'a>, opponent: &'a DeckBoard<'a>) -> House {
            self.house
        }

        fn next_action<'a, 'b>(&self, turn: &'b Turn<'a>) -> Option<Action<'a>> {
            None
        }
    }
}