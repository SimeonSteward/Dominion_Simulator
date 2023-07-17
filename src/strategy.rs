use crate::{
    card::{constants::*, Card, CardType},
    player::Player,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub struct CardCondition<'a> {
    pub card: &'a Card,
    pub condition: Option<fn(&Player) -> bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    EqualTo,
    NotEqualTo,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionValue {
    Int(u16),
    CountInDeck(String),
    CountTypeInDeck(CardType),
    CountInSupply(String),
    CountAllCardsInDeck,
    CountVp,
    CountOpponentVp,
    // Plus{
    //     first: &'a ConditionValue<'a>,
    //     second: &'a ConditionValue<'a>
    // }
}
#[derive(Serialize, Deserialize)]
pub struct Condition {
    condition_type: ConditionType,
    first: ConditionValue,
    second: ConditionValue,
}

#[derive(Serialize, Deserialize)]
pub struct NameCondition {
    name: String,
    condition: Condition,
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

pub fn get_buy_priority<'a>(path: String) -> Result<Vec<NameCondition>, std::io::Error> {
    // Open the file in read-only mode with buffer.
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    // Read the JSON contents of the file as an instance of `User`.
    let conds = serde_json::from_reader(reader)?;
    Ok(conds)
}

pub fn save_buy_priority(
    buy_priority: Vec<NameCondition>,
    name: String,
) -> Result<(), std::io::Error> {
    let mut file = match std::fs::File::create(name) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let buy_priority_str = serde_json::to_string(&buy_priority)?;
    let buy_priority_buf = buy_priority_str.as_bytes();
    std::io::Write::write_all(&mut file, buy_priority_buf)?;
    Ok(())
}
