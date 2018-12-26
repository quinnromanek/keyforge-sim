use reqwest;
use std::error;
use std::collections::HashMap;
use std::fs::File;
use game::{CardDetails, House};
use serde_json;
use std::fmt;


pub struct Deck {
    pub cards: Vec<String>,
    pub card_details: HashMap<String, CardDetails>,
    pub houses: Vec<House>,
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            card_details: HashMap::new(),
            houses: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct DeckLinks {
    cards: Vec<CardDetails>
}

#[derive(Deserialize, Serialize, Debug)]
struct DeckDataLinks {
    cards: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct DeckData {
    pub name: String,
    pub _links: DeckDataLinks,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeckResponse {
    data: DeckData,
    _linked: DeckLinks,

}

#[derive(Debug)]
pub struct DeckError;

impl fmt::Display for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Loading Deck Failed")
    }
}

impl error::Error for DeckError {
    fn description(&self) -> &str {
        "Loading Deck Failed"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub fn get_deck(deck_id: &str) -> Result<Deck, DeckError> {
    let file_path = format!("decks/{}.deck", deck_id);
    let deck_response = match File::open(&file_path) {
        Ok(file) => {
            let dr: DeckResponse = serde_json::from_reader(file).map_err(|_| DeckError)?;
            dr
        },
        Err(_) => {
            let request_url = format!("https://www.keyforgegame.com/api/decks/{}/?links=cards", deck_id);
            let mut response = reqwest::get(&request_url).map_err(|_| DeckError)?;
            let dr: DeckResponse = response.json().map_err(|_| DeckError)?;
            if let Ok(file) = File::create(file_path) {
                serde_json::to_writer(file, &dr).map_err(|_| DeckError)?;
            }
            dr
        }
    };
    let mut deck = Deck::new();
    for details in deck_response._linked.cards {
        let house = details.house;
        deck.card_details.insert(details.id.clone(), details);
        if !deck.houses.contains(&house) {
            deck.houses.push(house);
        }
    }
    for id in deck_response.data._links.cards {
        deck.cards.push(id)
    }
    assert_eq!(deck.houses.len(), 3);
    Ok(deck)
}

#[cfg(test)]
pub mod test {
    use game::CardDetails;
    use super::Deck;

    pub fn deck_from(card_details: CardDetails, count: u32) -> Deck {
        let mut deck = Deck::new();
        let id = card_details.id.clone();
        let house = card_details.house;
        deck.card_details.insert(card_details.id.clone(), card_details);
        for _ in 0..count {
            deck.cards.push(id.clone());
        }
        deck.houses.push(house);
        deck
    }
}
