use api::{Deck, CardDetails};
use rand::{thread_rng, seq::SliceRandom};

pub struct DeckBoard<'a> {
    deck_details: &'a Deck,
    pub hand: Vec<&'a CardDetails>,
    pub deck: Vec<&'a CardDetails>,
    pub discard: Vec<&'a CardDetails>,
}

impl<'a> DeckBoard<'a> {
    pub fn new(deck_details: &Deck) -> DeckBoard {
        let mut deck = DeckBoard {
            deck_details,
            hand: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
        };
        for details in deck.deck_details.card_details.values() {
            deck.deck.push(details);
        }
        deck.shuffle();
        deck.draw_to(6);
        deck
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }

    fn reshuffle_discard(&mut self) {
        self.deck.append(&mut self.discard);
        self.shuffle();
    }

    fn draw_card(&mut self) {
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

    fn draw_to(&mut self, limit: usize) {
        while self.hand.len() < limit {
            self.draw_card();
        }
    }
}