use crate::{
    card::{self, Card, CardType},
    kingdom::Kingdom,
    player::Player,
};
use serde::{Deserialize, Serialize};

pub struct CardCondition<'a> {
    pub card: &'a Card,
    pub condition: Box<dyn Send + Sync + Fn(&Player<'_>, &Player<'_>, &Kingdom<'_>) -> bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    True,
    EqualTo {
        first: ConditionValue,
        second: ConditionValue,
    },
    NotEqualTo {
        first: ConditionValue,
        second: ConditionValue,
    },
    GreaterThan {
        first: ConditionValue,
        second: ConditionValue,
    },
    GreaterThanOrEqualTo {
        first: ConditionValue,
        second: ConditionValue,
    },
    LessThan {
        first: ConditionValue,
        second: ConditionValue,
    },
    LessThanOrEqualTo {
        first: ConditionValue,
        second: ConditionValue,
    },
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
pub struct NameCondition {
    pub card: String,
    pub condition: ConditionType,
}

fn condition_to_func(condition: ConditionType) -> Box<dyn Send + Sync + Fn(&Player, &Player, &Kingdom) -> bool> {
    fn func(value: &ConditionValue) -> Box<dyn Fn(&Player, &Player, &Kingdom) -> u16 + '_> {
        match value {
            ConditionValue::Int(val) => Box::new(move |_player, _opponent, _kingdom| *val),
            ConditionValue::CountInDeck(card_name) => {
                Box::new(move |player, _opponent, _kingdom| -> u16 {
                    let card = card::constants::get_card(card_name);
                    player.cards.get(card).copied().unwrap_or(0)
                })
            }
            ConditionValue::CountTypeInDeck(card_type) => {
                Box::new(move |player, _opponent, _kingdom| -> u16 {
                    let mut sum: u16 = 0;
                    for (key, value) in player.cards.iter() {
                        if key.card_type == *card_type {
                            sum += value;
                        }
                    }
                    sum
                })
            }
            ConditionValue::CountInSupply(card_name) => {
                Box::new(move |_player, _opponent, kingdom| -> u16 {
                    let card = card::constants::get_card(card_name);
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
    let ret_val: Box<dyn Send + Sync + Fn(&Player, &Player, &Kingdom) -> bool> = match condition {
        ConditionType::True => Box::new(|_player, _opponent, _kingdom| true),
        ConditionType::EqualTo { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                    let f_func = func(&first);
                    let s_func = func(&second);
                    f_func(player, opponent, kingdom) == s_func(player, opponent, kingdom)
                })
        },
        ConditionType::NotEqualTo { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                    let f_func = func(&first);
                    let s_func = func(&second);
                    f_func(player, opponent, kingdom) != s_func(player, opponent, kingdom)
                })
        },
        ConditionType::GreaterThan { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                    let f_func = func(&first);
                    let s_func = func(&second);
                    f_func(player, opponent, kingdom) > s_func(player, opponent, kingdom)
                })
        },
        ConditionType::GreaterThanOrEqualTo { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                let f_func = func(&first);
                let s_func = func(&second);
                f_func(player, opponent, kingdom) >= s_func(player, opponent, kingdom)
            })
        }
        ConditionType::LessThan { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                    let f_func = func(&first);
                    let s_func = func(&second);
                    f_func(player, opponent, kingdom) < s_func(player, opponent, kingdom)
                })
        },
        ConditionType::LessThanOrEqualTo { first, second } => {
            Box::new(move |player, opponent, kingdom| {
                let f_func = func(&first);
                let s_func = func(&second);
                f_func(player, opponent, kingdom) <= s_func(player, opponent, kingdom)
            })
        }
    };
    ret_val
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
pub fn load_priority_list(name: String) -> Result<Vec<NameCondition>, std::io::Error> {
    let mut strategy_name = name.trim().to_string();
    strategy_name = format!("strategies/{strategy_name}.json").to_string();
    // Open the file in read-only mode with buffer.
    let file = std::fs::File::open(strategy_name)?;
    let reader = std::io::BufReader::new(file);
    // Read the JSON contents of the file as an instance of `User`.
    let conds = serde_json::from_reader(reader)?;
    Ok(conds)
}

pub fn get_priority_list(name:String) -> Result<Vec<CardCondition<'static>>,std::io::Error> {
    let named_priority_list = load_priority_list(name);
    match named_priority_list {
        Ok(named_p_list) => Ok(named_priority_to_card_priority(named_p_list)),
        Err(err) => Err(err),
    }

}

pub fn save_priority_list(
    priority_list: Vec<NameCondition>,
    name: String,
) -> Result<(), std::io::Error> {
    let mut strategy_name = name.trim().to_string();
    strategy_name = format!("strategies/{strategy_name}.json").to_string();
    let mut file = match std::fs::File::create(strategy_name) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let buy_priority_str = serde_json::to_string_pretty(&priority_list)?;
    let buy_priority_buf = buy_priority_str.as_bytes();
    std::io::Write::write_all(&mut file, buy_priority_buf)?;
    Ok(())
}
