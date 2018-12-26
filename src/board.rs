use api::Deck;
use game::{CardDetails, Type, Flank};
use rand::{thread_rng, seq::SliceRandom};
use field::{Artifact, Creature, FieldCard};

pub struct DeckBoard<'a> {
    pub deck_details: &'a Deck,

    // Card Reserves
    pub hand: Vec<&'a CardDetails>,
    pub deck: Vec<&'a CardDetails>,
    pub discard: Vec<&'a CardDetails>,

    // Field
    pub creatures: Vec<Creature<'a>>,
    pub artifacts: Vec<Artifact<'a>>,

    // Global Values
    pub amber: u32,
    pub keys: u32,
    pub chains: u32,
}

impl<'a> DeckBoard<'a> {
    pub fn new(deck_details: &Deck) -> DeckBoard {
        let mut deck = DeckBoard {
            deck_details,
            hand: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),

            creatures: Vec::new(),
            artifacts: Vec::new(),

            amber: 0,
            keys: 0,
            chains: 0,
        };
        for id in &deck.deck_details.cards {
            let details = &deck.deck_details.card_details[id];
            deck.deck.push(details);
        }
        deck.shuffle();
        deck.draw_to(6);
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }

    pub fn reshuffle_discard(&mut self) {
        self.deck.append(&mut self.discard);
        self.shuffle();
    }

    pub fn draw_card(&mut self) {
        loop {
            match self.deck.pop() {
                Some(card) => {
                    self.hand.push(card);
                    return;
                }
                None => self.reshuffle_discard()
            }
        }
    }

    pub fn discard(&mut self, index: usize) {
        let out = self.hand.remove(index);
        self.discard.push(out);
    }

    pub fn draw_to(&mut self, limit: usize) {
        while self.hand.len() < limit {
            self.draw_card();
        }
    }

    // In this function, we assume that the passed card is playable.
    // house choice and other effects should be tested before calling it
    pub fn play(&mut self, cd: &'a CardDetails, from_hand: bool, flank: Flank) {
        // Increment any immediate amber
        self.amber += cd.amber;

        if from_hand {
            // Remove the card from your hand if you played it from there
            self.hand.remove_item(&cd);
        }

        match cd.card_type {
            Type::Action => self.discard.push(cd),
            Type::Artifact => {
                self.artifacts.push(Artifact::new(cd))
            }
            Type::Upgrade => error!("Upgrades not yet implemented"),
            Type::Creature => self.play_creature(cd, flank),
        }
    }

    fn play_creature(&mut self, cd: &'a CardDetails, flank: Flank) {
        match flank {
            Flank::Left => {
                if self.creatures.len() > 1 {
                    if let Some(current_flank) = self.creatures.first_mut() {
                        current_flank.on_flank = false;
                    }
                }
                self.creatures.insert(0, Creature::new(cd));
            }
            Flank::Right => {
                if self.creatures.len() > 1 {
                    if let Some(current_flank) = &mut self.creatures.last_mut() {
                        current_flank.on_flank = false;
                    }
                }
                self.creatures.push(Creature::new(cd));
            }
        }
    }

    // Any destroyed effects should happen before this
    // By default, the creature is just purged, the card needs to be manually
    // Added to the discard
    pub fn destroy_creature(&mut self, index: usize) -> &'a CardDetails {
        let destroyed_creature = self.creatures.remove(index);
        if let Some(front) = self.creatures.first_mut() {
            front.on_flank = true;
        }

        if let Some(back) = self.creatures.last_mut() {
            back.on_flank = true;
        }
        destroyed_creature.details
    }

    pub fn my_turn_over(&mut self) {
        for c in &mut self.creatures {
            c.on_my_turn_over();
        }

        for a in &mut self.artifacts {
            a.on_my_turn_over();
        }
    }

    pub fn turn_over(&mut self) {
        for c in &mut self.creatures {
            c.on_turn_over();
        }

        for a in &mut self.artifacts {
            a.on_turn_over();
        }
    }
}

#[cfg(test)]
mod test {
    use ::board::DeckBoard;
    use api;
    use api::test::deck_from;
    use game::test::test_card;
    use game::{CardDetails, House, Type, Flank};

    struct TestFixture<'a> {
        pub deck_board: DeckBoard<'a>
    }

    fn fixture<F>(tf: F) where F: Fn(&mut TestFixture) {
        let deck = api::get_deck("test").unwrap();

        let deck_board = DeckBoard::new(&deck);
        let mut o = TestFixture { deck_board };
        tf(&mut o);
    }

    fn deck_fixture<F>(details: CardDetails, tf: F) where F: Fn(&mut TestFixture) {
        let deck = deck_from(details, 36);
        let deck_board = DeckBoard::new(&deck);
        let mut o = TestFixture { deck_board };
        tf(&mut o);
    }

    #[test]
    fn test_starting_deck() {
        fixture(|f| {
            assert_eq!(f.deck_board.deck.len(), 30);
            assert_eq!(f.deck_board.hand.len(), 6);
            assert_eq!(f.deck_board.discard.len(), 0);
        });
    }

    #[test]
    fn test_discard() {
        fixture(|f| {
            f.deck_board.discard(2);
            assert_eq!(f.deck_board.hand.len(), 5);
            assert_eq!(f.deck_board.discard.len(), 1);
        });
    }

    #[test]
    fn test_reshuffle() {
        fixture(|f| {
            let deck_len = f.deck_board.deck.len();
            f.deck_board.draw_to(deck_len + 6);
            assert_eq!(f.deck_board.deck.len(), 0);
            for _ in 0..10 {
                f.deck_board.discard(0);
            }
            assert_eq!(f.deck_board.discard.len(), 10);
            f.deck_board.draw_card();
            assert_eq!(f.deck_board.discard.len(), 0);
            assert_eq!(f.deck_board.deck.len(), 9);
        });
    }

    #[test]
    fn test_play_creature() {
        let card = test_card(House::Brobnar, Type::Creature, 0, 2, 0);
        deck_fixture(card, |f| {
            assert_eq!(f.deck_board.hand.len(), 6);
            assert_eq!(f.deck_board.creatures.len(), 0);
            assert_eq!(f.deck_board.hand.first().unwrap().card_type, Type::Creature);
            let &card = f.deck_board.hand.first().unwrap();
            f.deck_board.play(card, true, Flank::Left);
            assert_eq!(f.deck_board.creatures.len(), 1);
            assert_eq!(f.deck_board.hand.len(), 5);
            f.deck_board.play(card, true, Flank::Left);
            f.deck_board.play(card, true, Flank::Left);
            assert_eq!(f.deck_board.creatures.len(), 3);

            assert_eq!(f.deck_board.creatures[0].on_flank, true);
            assert_eq!(f.deck_board.creatures[1].on_flank, false);
            assert_eq!(f.deck_board.creatures[2].on_flank, true);
        });
    }

    #[test]
    fn test_destroy_creature() {
        let card = test_card(House::Brobnar, Type::Creature, 0, 2, 0);
        deck_fixture(card, |f| {
            let &card = f.deck_board.hand.first().unwrap();
            f.deck_board.play(card, true, Flank::Left);
            f.deck_board.play(card, true, Flank::Left);
            f.deck_board.play(card, true, Flank::Left);
            assert_eq!(f.deck_board.creatures[0].on_flank, true);
            assert_eq!(f.deck_board.creatures[1].on_flank, false);
            assert_eq!(f.deck_board.creatures[2].on_flank, true);
            f.deck_board.destroy_creature(0);
            assert_eq!(f.deck_board.creatures.len(), 2);
            assert_eq!(f.deck_board.creatures[0].on_flank, true);
            assert_eq!(f.deck_board.creatures[1].on_flank, true);
        });
    }
}
