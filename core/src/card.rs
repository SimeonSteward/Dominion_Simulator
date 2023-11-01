use serde::{Deserialize, Serialize};

use crate::player;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CardType {
    Treasure,
    Action,
    Victory,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Clone)]
pub struct VictoryType {
    pub vp: u16,
}

pub struct Card {
    pub name: &'static str,
    pub cost: u16,
    pub card_type: CardType,
    pub play_action: fn(&mut player::Player),
    pub play_treasure: fn(&mut player::Player, u16),
    pub points: fn(&player::Player) -> u16,
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Card {}
impl std::hash::Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Default for Card {
    fn default() -> Self {
        Card {
            name: "",
            cost: u16::MAX,
            card_type: CardType::Action,
            play_action: |_| {},
            play_treasure: |_, _| {},
            points: |_| 0,
        }
    }
}

pub mod constants {
    use std::collections::HashMap;

    use crate::card::{Card, CardType};
    use lazy_static::lazy_static;

    pub fn get_card(card_name: &String) -> &'static Card {
        match CARD_MAP.get(&card_name.to_lowercase()) {
            Some(card) => card,
            None => {
                panic!("Card {} not found", card_name);
            }
        }
    }

    pub fn is_card(card_name: &str) -> bool {
        CARD_MAP.contains_key(&card_name.to_lowercase())
    }

    lazy_static! {
        pub static ref COPPER: Card = Card {
            name: "Copper",
            cost: 0,
            card_type: CardType::Treasure,
            play_treasure: |player, n| { player.coins += n },
            ..Default::default()
        };
        pub static ref SILVER: Card = Card {
            name: "Silver",
            cost: 3,
            card_type: CardType::Treasure,
            play_treasure: |player, n| { player.coins += 2 * n },
            ..Default::default()
        };
        pub static ref GOLD: Card = Card {
            name: "Gold",
            cost: 6,
            card_type: CardType::Treasure,
            play_treasure: |player, n| { player.coins += 3 * n },
            ..Default::default()
        };
        pub static ref ESTATE: Card = Card {
            name: "Estate",
            cost: 2,
            card_type: CardType::Victory,
            points: |_| { 1 },
            ..Default::default()
        };
        pub static ref DUCHY: Card = Card {
            name: "Duchy",
            cost: 5,
            card_type: CardType::Victory,
            points: |_| { 3 },
            ..Default::default()
        };
        pub static ref PROVINCE: Card = Card {
            name: "Province",
            cost: 8,
            card_type: CardType::Victory,
            points: |_| { 6 },
            ..Default::default()
        };
        pub static ref VILLAGE: Card = Card {
            name: "Village",
            cost: 3,
            card_type: CardType::Action,
            play_action: |player| {
                player.draw(1);
                player.actions += 2;
            },
            ..Default::default()
        };
        pub static ref SMITHY: Card = Card {
            name: "Smithy",
            cost: 4,
            card_type: CardType::Action,
            play_action: |player| {
                player.draw(3);
            },
            ..Default::default()
        };
        pub static ref MARKET: Card = Card {
            name: "Market",
            cost: 5,
            card_type: CardType::Action,
            play_action: |player| {
                player.draw(1);
                player.actions += 1;
                player.coins += 1;
                player.buys += 1;
            },
            ..Default::default()
        };
        pub static ref FESTIVAL: Card = Card {
            name: "Festival",
            cost: 5,
            card_type: CardType::Action,
            play_action: |player| {
                player.actions += 2;
                player.coins += 2;
                player.buys += 1;
            },
            ..Default::default()
        };
        pub static ref LABORATORY: Card = Card {
            name: "Laboratory",
            cost: 5,
            card_type: CardType::Action,
            play_action: |player| {
                player.draw(2);
                player.actions += 1;
            },
            ..Default::default()
        };
        static ref CARD_MAP: HashMap<String, &'static Card> = {
            let mut map = HashMap::<String, &'static Card>::new();

            macro_rules! add_card {
                ($card:expr) => {
                    map.insert($card.name.to_string().to_lowercase(), &$card);
                };
            }

            add_card!(PROVINCE);
            add_card!(DUCHY);
            add_card!(ESTATE);
            add_card!(GOLD);
            add_card!(SILVER);
            add_card!(COPPER);
            add_card!(VILLAGE);
            add_card!(SMITHY);
            add_card!(MARKET);
            add_card!(FESTIVAL);
            add_card!(LABORATORY);

            map
        };
    }
}
