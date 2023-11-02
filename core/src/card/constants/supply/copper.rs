use lazy_static::lazy_static;
use crate::card::{Card, CardType};
lazy_static! {
    pub static ref COPPER: Card = Card {
        name: "Copper",
        cost: 0,
        card_type: CardType::Treasure,
        coin: 1,
        play_treasure: |player, n| { player.coins += n },
        ..Default::default()
    };
}
