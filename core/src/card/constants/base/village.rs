use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref VILLAGE: Card = Card {
        name: "Village",
        cost: 3,
        card_type: CardType::Action,
        play_action: |player, _| {
            player.draw(1);
            player.actions += 2;
        },
        ..Default::default()
    };
}
