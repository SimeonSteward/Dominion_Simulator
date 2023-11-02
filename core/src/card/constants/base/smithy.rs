use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref SMITHY: Card = Card {
        name: "Smithy",
        cost: 4,
        card_type: CardType::Action,
        play_action: |player, _| {
            player.draw(3);
        },
        ..Default::default()
    };
}
