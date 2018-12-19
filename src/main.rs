#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate rand;

mod api;
mod deck;

static DECK_ID: &str = "8339ebd0-bd62-47b5-9274-37394546b238";

fn main() {
    match api::get_deck(DECK_ID) {
        Ok(deck_details) => {
            let mut deck = deck::DeckBoard::new(&deck_details);
            for card in deck.hand {
                    println!("{house} | {card_title} | {card_type} ", card_title = card.card_title, house = card.house, card_type = card.card_type);
            }
        }
        Err(err) => {
            panic!("Didn't get deck: {}", err)
        }
    }
}
