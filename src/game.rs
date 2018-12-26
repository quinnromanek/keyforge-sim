use std::cmp::PartialEq;

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
pub enum House {
    Brobnar,
    Dis,
    Logos,
    Mars,
    Sanctum,
    Shadows,
    Untamed,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Action,
    Artifact,
    Creature,
    Upgrade,
}

pub enum Flank {
    Left,
    Right,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardDetails {
    pub id: String,
    pub card_title: String,
    pub house: House,
    pub card_type: Type,
    pub front_image: String,
    pub card_text: Option<String>,
    pub traits: Option<String>,
    pub amber: u32,
    pub power: u32,
    pub armor: u32,
    pub flavor_text: Option<String>,
}

impl PartialEq for CardDetails {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
pub mod test {
    use super::{CardDetails, House, Type};
    use uuid::Uuid;

    pub fn test_card(house: House, card_type: Type, amber: u32, power: u32, armor: u32) -> CardDetails {
        CardDetails {
            id: Uuid::new_v4().to_string(),
            card_title: "test".to_string(),
            house,
            card_type,
            front_image: "".to_string(),
            card_text: None,
            traits: None,
            amber,
            power,
            armor,
            flavor_text: None,
        }
    }
}
