use game::{House, CardDetails, Flank};
use field::Creature;
use board::DeckBoard;
use player::Player;

pub struct Turn<'a> {
    mine: DeckBoard<'a>,
    opponent: DeckBoard<'a>,
    house: House,
    player: Box<dyn Player>,
}

pub enum Action<'a> {
    Play(&'a CardDetails, Flank),
    Reap(usize),
    Fight(usize, usize),
}


impl<'a> Turn<'a> {
    fn new(mine: DeckBoard<'a>, opponent: DeckBoard<'a>, player: Box<dyn Player>) -> Self {
        let house = player.choose_house(&mine, &opponent);
        Turn {
            mine,
            opponent,
            house,
            player,
        }
    }

    pub fn can_play(&self, card_details: &'a CardDetails) -> bool {
        card_details.house == self.house
    }

    fn play(&mut self, card: &'a CardDetails, from_hand: bool, flank: Flank) {
        if self.can_play(card) {
            self.mine.play(card, from_hand, flank);
        }
    }

    fn reap<'b>(&mut self, index: usize) {
        let creature = &mut self.mine.creatures[index];
        assert!(!creature.exhausted);
        creature.exhausted = true;
        self.mine.amber += 1;
    }

    fn fight(&mut self, my_index: usize, target_index: usize) {
        assert!(!self.mine.creatures[my_index].exhausted);
        {
            self.mine.creatures[my_index].exhausted = true;
            self.mine.creatures[my_index].fight(&mut self.opponent.creatures[target_index]);
        }

        if !self.mine.creatures[my_index].is_alive() {
            let destroyed_creature = self.mine.destroy_creature(my_index);
            self.mine.discard.push(destroyed_creature);
        }

        if !self.opponent.creatures[target_index].is_alive() {
            let destroyed_creature = self.opponent.destroy_creature(target_index);
            self.opponent.discard.push(destroyed_creature);
        }
    }

    fn execute_action(&mut self, action: Action<'a>) {
        match action {
            Action::Play(details, flank) => self.play(details, true, flank),
            Action::Reap(index) => self.reap(index),
            Action::Fight(this, other) => self.fight(this, other),
        }
    }

    pub fn run(&mut self) {
        while let Some(action) = self.player.next_action(self) {
            self.execute_action(action);
        }
    }

    pub fn end(&mut self) {
        self.mine.my_turn_over();
        self.mine.turn_over();
        self.opponent.turn_over();
    }
}

#[cfg(test)]
pub mod test {
    use super::{Turn, Action};
    use api::test::deck_from;
    use game::CardDetails;
    use board::DeckBoard;
    use player::test::TestPlayer;
    use game::test::test_card;
    use game::{House, Type, Flank};

    fn board_fixture<F>(details_a: CardDetails, details_b: CardDetails, tf: F) where F: Fn(&mut Turn) {
        let deck_a = deck_from(details_a, 36);
        let deck_b = deck_from(details_b, 36);
        let mut board_a = DeckBoard::new(&deck_a);
        let mut board_b = DeckBoard::new(&deck_b);
        let player = TestPlayer { house: *deck_a.houses.first().unwrap() };
        let mut turn = Turn::new(board_a, board_b, Box::new(player));
        tf(&mut turn);
    }

    #[test]
    fn test_turn_play_creature() {
        let card = test_card(House::Brobnar, Type::Creature, 0, 2, 0);
        board_fixture(card.clone(), card, |turn| {
            let &card = turn.mine.hand.first().unwrap();
            assert!(turn.can_play(card));
            let play_action = Action::Play(card, Flank::Right);
            turn.execute_action(play_action);
            assert_eq!(turn.mine.creatures.len(), 1);
            assert_eq!(turn.mine.hand.len(), 5);
            assert_eq!(turn.mine.discard.len(), 0);
        });
    }

    #[test]
    fn test_turn_reap_creature() {
        let card = test_card(House::Brobnar, Type::Creature, 0, 2, 0);
        board_fixture(card.clone(), card, |turn| {
            let &card = turn.mine.hand.first().unwrap();
            let play_action = Action::Play(card, Flank::Right);
            turn.execute_action(play_action);
            let reap_action = Action::Reap(0);
            turn.end();
            turn.execute_action(reap_action);
            assert_eq!(turn.mine.amber, 1);
            assert!(turn.mine.creatures[0].exhausted);
        });
    }

    #[test]
    fn test_turn_fight_creature() {
        let card = test_card(House::Brobnar, Type::Creature, 0, 2, 0);
        let card2 = test_card(House::Brobnar, Type::Creature, 0, 1, 0);
        board_fixture(card, card2, |turn| {
            let &card = turn.mine.hand.first().unwrap();
            let play_action = Action::Play(card, Flank::Right);
            turn.execute_action(play_action);
            let &card2 = turn.opponent.hand.first().unwrap();
            turn.opponent.play(card2, true, Flank::Right);
            assert_eq!(turn.opponent.creatures.len(), 1);
            turn.end();
            let fight_action = Action::Fight(0, 0);
            turn.execute_action(fight_action);
            assert_eq!(turn.opponent.creatures.len(), 0);
            assert_eq!(turn.opponent.discard.len(), 1);
            assert_eq!(turn.opponent.discard[0], card2);
            assert_eq!(turn.mine.creatures.len(), 1);
            assert_eq!(turn.mine.creatures[0].damage, 1);
            assert!(turn.mine.creatures[0].is_alive());
            assert!(turn.mine.creatures[0].exhausted);
        });
    }
}
