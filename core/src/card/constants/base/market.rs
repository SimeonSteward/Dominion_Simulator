use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref MARKET: Card = Card {
            name: "Market",
            cost: 5,
            card_type: CardType::Action,
            play_action: |player, _| {
                player.draw(1);
                player.actions += 1;
                player.coins += 1;
                player.buys += 1;
            },
            ..Default::default()
        };
}
