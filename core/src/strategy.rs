use crate::{
    card::{self, Card, CardType},
    kingdom::Kingdom,
    player::Player,
};
use serde::{Deserialize, Serialize};

pub struct CardCondition<'a> {
    pub card: &'a Card,
    pub condition: Box<dyn Fn(&Player<'_>, &Player<'_>, &Kingdom<'_>) -> bool>,
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
    pub condition_type: ConditionType,
    pub first: ConditionValue,
    pub second: ConditionValue,
}

#[derive(Serialize, Deserialize)]
pub struct NameCondition {
    pub card: String,
    pub condition: Condition,
}

fn condition_to_func(condition: Condition) -> Box<dyn Fn(&Player, &Player, &Kingdom) -> bool> {
    let comparator: fn(first: u16, last: u16) -> bool = match condition.condition_type {
        ConditionType::GreaterThan => |first: u16, last: u16| first > last,
        ConditionType::GreaterThanOrEqualTo => |first: u16, last: u16| first >= last,
        ConditionType::LessThan => |first: u16, last: u16| first < last,
        ConditionType::LessThanOrEqualTo => |first: u16, last: u16| first <= last,
        ConditionType::EqualTo => |first: u16, last: u16| first == last,
        ConditionType::NotEqualTo => |first: u16, last: u16| first != last,
    };
    fn func(value: ConditionValue) -> Box<dyn Fn(&Player, &Player, &Kingdom) -> u16> {
        match value {
            ConditionValue::Int(val) => Box::new(move |_player, _opponent, _kingdom| val),
            ConditionValue::CountInDeck(card_name) => {
                Box::new(move |player, _opponent, _kingdom| -> u16 {
                    let card = card::constants::get_card(&card_name);
                    match player.cards.get(card).copied() {
                        Some(num) => num,
                        None => 0,
                    }
                })
            }
            ConditionValue::CountTypeInDeck(card_type) => {
                Box::new(move |player, _opponent, _kingdom| -> u16 {
                    let mut sum: u16 = 0;
                    for (key, value) in player.cards.iter() {
                        if key.card_type == card_type {
                            sum += value;
                        }
                    }
                    sum
                })
            }
            ConditionValue::CountInSupply(card_name) => {
                Box::new(move |_player, _opponent, kingdom| -> u16 {
                    let card = card::constants::get_card(&card_name);
                    let supply_pile = kingdom.supply_piles.get(card);
                    match supply_pile {
                        Some(supply_pile) => supply_pile.count,
                        None => 0,
                    }
                })
            }

            ConditionValue::CountAllCardsInDeck => Box::new(|player, _opponent, _kingdom| -> u16 {
                let mut sum = 0;
                for pair in &player.cards {
                    sum += pair.1;
                }
                sum
            }),
            ConditionValue::CountVp => {
                Box::new(|player, _opponent, _kingdom| -> u16 { player.get_vp() })
            }
            ConditionValue::CountOpponentVp => {
                Box::new(|_player, opponent, _kingdom| -> u16 { opponent.get_vp() })
            }
        }
    }
    let first = func(condition.first);
    let second = func(condition.second);
    Box::new(move |player, opponent, kingdom| {
        comparator(
            first(player, opponent, kingdom),
            second(player, opponent, kingdom),
        )
    })
}

pub fn named_priority_to_card_priority(
    named_priority: Vec<NameCondition>,
) -> Vec<CardCondition<'static>> {
    let mut card_priority: Vec<CardCondition> = Vec::new();
    for priority in named_priority {
        let card = card::constants::get_card(&priority.card.to_string());
        let condition = condition_to_func(priority.condition);
        card_priority.push(CardCondition { card, condition });
    }
    card_priority
}
pub fn get_priority_list(path: String) -> Result<Vec<NameCondition>, std::io::Error> {
    // Open the file in read-only mode with buffer.
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    // Read the JSON contents of the file as an instance of `User`.
    let conds = serde_json::from_reader(reader)?;
    Ok(conds)
}

pub fn save_priority_list(
    priority_list: Vec<NameCondition>,
    name: String,
) -> Result<(), std::io::Error> {
    let mut file = match std::fs::File::create(name) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let buy_priority_str = serde_json::to_string(&priority_list)?;
    let buy_priority_buf = buy_priority_str.as_bytes();
    std::io::Write::write_all(&mut file, buy_priority_buf)?;
    Ok(())
}
