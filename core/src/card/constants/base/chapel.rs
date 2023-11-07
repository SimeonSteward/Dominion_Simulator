use crate::card::{Card, CardType};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref CHAPEL: Card = Card {
        name: "Chapel",
        cost: 2,
        card_type: CardType::Action,
        play_action: |player, opponent, kingdom| {
          player.trash_up_to_n_cards_in_hand(opponent, kingdom, 4)
        },
        ..Default::default()
    };
}
