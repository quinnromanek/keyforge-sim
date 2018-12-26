#![feature(vec_remove_item)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate rand;
extern crate uuid;

mod game;
mod api;
mod board;
mod field;
mod turn;
mod player;


static DECK_ID: &str = "8339ebd0-bd62-47b5-9274-37394546b238";

struct Game {
    first_player: Box<dyn player::Player>,

}

fn main() {
    match api::get_deck(DECK_ID) {
        Ok(deck_details) => {
            println!("Houses: {:?}", deck_details.houses);
            let mut deck = board::DeckBoard::new(&deck_details);
            for card in deck.hand {
                println!("{house:?} | {card_title} | {card_type:?} ", card_title = card.card_title, house = card.house, card_type = card.card_type);
            }
        }
        Err(err) => {
            panic!("Didn't get deck: {}", err)
        }
    }
}
