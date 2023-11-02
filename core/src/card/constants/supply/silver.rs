use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref SILVER: Card = Card {
        name: "Silver",
        cost: 3,
        card_type: CardType::Treasure,
        play_treasure: |player, n| { player.coins += 2 * n },
        ..Default::default()
    };
}
