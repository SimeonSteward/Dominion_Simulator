use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref ESTATE: Card = Card {
        name: "Estate",
        cost: 2,
        card_type: CardType::Victory,
        points: |_| { 1 },
        ..Default::default()
    };
}
