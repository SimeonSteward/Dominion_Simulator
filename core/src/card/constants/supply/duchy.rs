use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref DUCHY: Card = Card {
        name: "Duchy",
        cost: 5,
        card_type: CardType::Victory,
        points: |_| { 3 },
        ..Default::default()
    };
}
