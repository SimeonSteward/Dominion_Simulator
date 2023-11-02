use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref LABORATORY: Card = Card {
        name: "Laboratory",
        cost: 5,
        card_type: CardType::Action,
        play_action: |player| {
            player.draw(2);
            player.actions += 1;
        },
        ..Default::default()
    };
}
