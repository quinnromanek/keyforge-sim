use reqwest;
use std::collections::HashMap;


#[derive(Deserialize)]
struct Card {}

#[derive(Deserialize, Debug)]
pub struct CardDetails {
    pub id: String,
    pub card_title: String,
    pub house: String,
    pub card_type: String,
    pub front_image: String,
    pub card_text: Option<String>,
    pub traits: Option<String>,
    pub amber: u32,
    pub power: u32,
    pub rarity: String,
    pub flavor_text: Option<String>,
}

pub struct Deck {
    pub cards: Vec<String>,
    pub card_details: HashMap<String, CardDetails>,
}

impl Deck {
    fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            card_details: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct DeckLinks {
    cards: Vec<CardDetails>
}

#[derive(Deserialize, Debug)]
struct DeckDataLinks {
    cards: Vec<String>
}

#[derive(Deserialize, Debug)]
struct DeckData {
    pub name: String,
    pub _links: DeckDataLinks,
}

#[derive(Deserialize, Debug)]
pub struct DeckResponse {
    data: DeckData,
    _linked: DeckLinks,

}


pub fn get_deck(deck_id: &str) -> Result<Deck, reqwest::Error> {
    let request_url = format!("https://www.keyforgegame.com/api/decks/{}/?links=cards", deck_id);
    let mut response = reqwest::get(&request_url)?;
    let dr: DeckResponse = response.json()?;
    let mut deck = Deck::new();
    for details in dr._linked.cards {
        deck.card_details.insert(details.id.clone(), details);
    }
    for id in dr.data._links.cards {
        deck.cards.push(id)
    }
    Ok(deck)
}
