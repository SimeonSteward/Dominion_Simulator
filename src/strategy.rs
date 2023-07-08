use crate::{
    card::{constants::*, Card},
    player::Player,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub struct CardCondition<'a> {
    pub card: &'a Card,
    pub condition: Option<fn(&Player) -> bool>,
}

lazy_static! {
    pub static ref TREASURE_PLAY_PRIORITY_LIST: [CardCondition<'static>; 3] = {
        [
            CardCondition {
                card: &COPPER,
                condition: None,
            },
            CardCondition {
                card: &SILVER,
                condition: None,
            },
            CardCondition {
                card: &GOLD,
                condition: None,
            },
        ]
    };
    pub static ref ACTION_PLAY_PRIORITY_LIST: [CardCondition<'static>; 4] = {
        [
            CardCondition {
                card: &VILLAGE,
                condition: None,
            },
            CardCondition {
                card: &MARKET,
                condition: None,
            },
            CardCondition {
                card: &FESTIVAL,
                condition: None,
            },
            CardCondition {
                card: &SMITHY,
                condition: None,
            },
        ]
    };
    pub static ref BUY_PRIORITY: [CardCondition<'static>; 7] = {
        [
            CardCondition {
                card: &PROVINCE,
                condition: Some(|player: &Player| -> bool {
                    player.cards.get(&*GOLD).map_or(false, |count| *count >= 2)
                }),
            },
            CardCondition {
                card: &DUCHY,
                condition: Some(|player: &Player| -> bool {
                    player
                        .cards
                        .get(&*PROVINCE)
                        .map_or(false, |count| *count > 2)
                }),
            },
            CardCondition {
                card: &ESTATE,
                condition: Some(|player: &Player| -> bool {
                    player
                        .cards
                        .get(&*PROVINCE)
                        .map_or(false, |count| *count > 5)
                }),
            },
            CardCondition {
                card: &GOLD,
                condition: None,
            },
            CardCondition {
                card: &SMITHY,
                condition: Some(|player: &Player| -> bool {
                    player.cards.get(&*SMITHY).map_or(true, |count| *count < 2)
                }),
            },
            CardCondition {
                card: &SILVER,
                condition: None,
            },
            CardCondition {
                card: &COPPER,
                condition: None,
            },
        ]
    };
}

// pub fn getBuyPriority() -> Vec<CardCondition> {}

// pub fn saveBuyPriority(buy_priority: Vec<CardCondition>, name: std::string::String) {}
