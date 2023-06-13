#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    Treasure(TreasureType),
    Action(ActionType),
    Victory(VictoryType),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TreasureType {
    pub coin: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ActionType {
    pub plus_card: u16,
    pub plus_action: u16,
    pub plus_buy: u16,
    pub plus_coin: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VictoryType {
    pub vp: u16,
}

impl Default for TreasureType {
    fn default() -> Self {
        TreasureType { coin: 0 }
    }
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType {
            plus_card: 0,
            plus_action: 0,
            plus_buy: 0,
            plus_coin: 0,
        }
    }
}

impl Default for VictoryType {
    fn default() -> Self {
        VictoryType { vp: 0 }
    }
}

impl Default for CardType {
    fn default() -> Self {
        CardType::Treasure(TreasureType::default())
    }
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
            card_type: CardType::Treasure(crate::card::TreasureType {
                coin: 1,
                ..Default::default()
            }),
        };
        pub static ref SILVER: Card = Card {
            name: "Silver",
            cost: 3,
            card_type: CardType::Treasure(crate::card::TreasureType {
                coin: 2,
                ..Default::default()
            }),
        };
        pub static ref GOLD: Card = Card {
            name: "Gold",
            cost: 6,
            card_type: CardType::Treasure(crate::card::TreasureType {
                coin: 3,
                ..Default::default()
            }),
        };
        pub static ref ESTATE: Card = Card {
            name: "Estate",
            cost: 2,
            card_type: CardType::Victory(crate::card::VictoryType {
                vp: 1,
                ..Default::default()
            }),
        };
        pub static ref DUCHY: Card = Card {
            name: "Duchy",
            cost: 5,
            card_type: CardType::Victory(crate::card::VictoryType {
                vp: 3,
                ..Default::default()
            }),
        };
        pub static ref PROVINCE: Card = Card {
            name: "Province",
            cost: 8,
            card_type: CardType::Victory(crate::card::VictoryType {
                vp: 6,
                ..Default::default()
            }),
        };
        pub static ref VILLAGE: Card = Card {
            name: "Village",
            cost: 3,
            card_type: CardType::Action(crate::card::ActionType {
                plus_card: 1,
                plus_action: 2,
                ..Default::default()
            }),
        };
        pub static ref SMITHY: Card = Card {
            name: "Smithy",
            cost: 4,
            card_type: CardType::Action(crate::card::ActionType {
                plus_card: 3,
                ..Default::default()
            }),
        };
        pub static ref MARKET: Card = Card {
            name: "Market",
            cost: 5,
            card_type: CardType::Action(crate::card::ActionType {
                plus_card: 1,
                plus_action: 1,
                plus_buy: 1,
                plus_coin: 1,
                ..Default::default()
            }),
        };
        pub static ref FESTIVAL: Card = Card {
            name: "Festival",
            cost: 5,
            card_type: CardType::Action(crate::card::ActionType {
                plus_action: 2,
                plus_buy: 1,
                plus_coin: 2,
                ..Default::default()
            }),
        };
        pub static ref LABORATORY: Card = Card {
            name: "Laboratory",
            cost: 5,
            card_type: CardType::Action(crate::card::ActionType {
                plus_card: 2,
                plus_action: 1,
                ..Default::default()
            }),
        };
    }
}
