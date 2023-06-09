use crate::{card::{Card, constants::*}, player::Player};
use lazy_static::lazy_static;

pub struct CardCondition<'a> {
    pub card: &'a Card,
    pub condition: Option<fn(&Player) -> bool>,
}


lazy_static! {
  pub static ref TREASURE_PLAY_PRIORITY: [CardCondition<'static>; 3] = {
        [
            CardCondition {
                card: &*COPPER,
                condition: None,
            },
            CardCondition {
                card: &*SILVER,
                condition: None,
            },
            CardCondition {
                card: &*GOLD,
                condition: None,
            },
        ]
    };
}