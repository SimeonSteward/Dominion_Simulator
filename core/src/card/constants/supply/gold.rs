use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref GOLD: Card = Card {
        name: "Gold",
        cost: 6,
        card_type: CardType::Treasure,
        coin: 3,
        play_treasure: |player, n| { player.coins += 3 * n },
        ..Default::default()
    };
}
