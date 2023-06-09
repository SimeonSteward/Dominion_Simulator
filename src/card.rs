#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    Treasure {
        coin: u16,
    },
    Action {
        plus_card: u16,
        plus_action: u16,
        plus_buy: u16,
        plus_coin: u16,
    },
    Victory {
        vp: u16,
    },
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub name: &'static str,
    pub cost: u16,
    pub card_type: CardType,
}
pub mod constants {
    use crate::card::{Card, CardType};
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref COPPER: Card = Card {
            name: "Copper",
            cost: 0,
            card_type: CardType::Treasure { coin: 1 },
        };
        pub static ref SILVER: Card = Card {
            name: "Silver",
            cost: 3,
            card_type: CardType::Treasure { coin: 2 },
        };
        pub static ref GOLD: Card = Card {
            name: "Gold",
            cost: 6,
            card_type: CardType::Treasure { coin: 3 },
        };
        pub static ref ESTATE: Card = Card {
            name: "Estate",
            cost: 2,
            card_type: CardType::Victory { vp: 1 },
        };
        pub static ref DUCHY: Card = Card {
            name: "Duchy",
            cost: 5,
            card_type: CardType::Victory { vp: 3 },
        };
        pub static ref PROVINCE: Card = Card {
            name: "Province",
            cost: 8,
            card_type: CardType::Victory { vp: 6 },
        };
    }
}
