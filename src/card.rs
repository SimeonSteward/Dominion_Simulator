use lazy_static::lazy_static;

#[derive(Default)]
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

    pub static ref ESTATE: Card = Card {
        name: "Estate",
        cost: 2,
        vp: 1,
        ..Default::default()
    };
}
