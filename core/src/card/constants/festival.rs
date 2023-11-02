use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref FESTIVAL: Card = Card {
        name: "Festival",
        cost: 5,
        card_type: CardType::Action,
        play_action: |player| {
            player.actions += 2;
            player.coins += 2;
            player.buys += 1;
        },
        ..Default::default()
    };
}
