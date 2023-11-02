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

    use crate::card::Card;
    use lazy_static::lazy_static;
    pub mod supply {
        pub mod copper;
        pub mod duchy;
        pub mod estate;
        pub mod gold;
        pub mod province;
        pub mod silver;
    }
    pub mod base {
        pub mod festival;
        pub mod laboratory;
        pub mod market;
        pub mod smithy;
        pub mod village;
    }

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
        static ref CARD_MAP: HashMap<String, &'static Card> = {
            let mut map = HashMap::<String, &'static Card>::new();

            macro_rules! add_card {
                ($card:expr) => {
                    map.insert($card.name.to_string().to_lowercase(), &$card);
                };
            }

            add_card!(supply::province::PROVINCE);
            add_card!(supply::duchy::DUCHY);
            add_card!(supply::estate::ESTATE);
            add_card!(supply::gold::GOLD);
            add_card!(supply::silver::SILVER);
            add_card!(supply::copper::COPPER);
            add_card!(base::village::VILLAGE);
            add_card!(base::smithy::SMITHY);
            add_card!(base::market::MARKET);
            add_card!(base::festival::FESTIVAL);
            add_card!(base::laboratory::LABORATORY);

            map
        };
    }
}
