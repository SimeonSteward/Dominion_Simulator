use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref PROVINCE: Card = Card {
        name: "Province",
        cost: 8,
        card_type: CardType::Victory,
        points: |_| { 6 },
        ..Default::default()
    };
}
