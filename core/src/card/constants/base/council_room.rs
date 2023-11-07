use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref COUNCIL_ROOM: Card = Card {
        name: "Council Room",
        cost: 5,
        card_type: CardType::Action,
        play_action: |player, opponent, _| {
            player.draw(4);
            player.buys += 1;
            opponent.draw(1);
        },
        ..Default::default()
    };
}