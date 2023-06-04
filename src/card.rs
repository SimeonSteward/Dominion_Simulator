use lazy_static::lazy_static;

#[derive(Default, Debug, Eq, PartialEq, Hash)]
pub struct Card {
    pub name: &'static str,
    pub cost: u8,
    pub vp: u8,
    pub coin: u8,
    // type: cardType
}

lazy_static! {
    pub static ref COPPER: Card = Card {
        name: "Copper",
        coin: 1,
        ..Default::default()
    };
    pub static ref SILVER: Card = Card {
        name: "Silver",
        coin: 2,
        cost: 3,
        ..Default::default()
    };
    pub static ref GOLD: Card = Card {
        name: "Gold",
        coin: 3,
        cost: 6,
        ..Default::default()
    };
    pub static ref ESTATE: Card = Card {
        name: "Estate",
        cost: 2,
        vp: 1,
        ..Default::default()
    };
    pub static ref DUCHY: Card = Card {
        name: "Duchy",
        cost: 5,
        vp: 3,
        ..Default::default()
    };
    pub static ref PROVINCE: Card = Card {
        name: "Province",
        cost: 8,
        vp: 6,
        ..Default::default()
    };
}
