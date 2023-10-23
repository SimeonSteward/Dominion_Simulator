use crate::{card, strategy};

pub fn create_new_priority() {
    let mut strategy_name = String::new(); // Declare input here
    println!("Enter Strategy Name:");
    std::io::stdin()
        .read_line(&mut strategy_name)
        .expect("Failed to read line");
    let priority_list = user_input_to_priority_list();
    let _ = strategy::save_priority_list(priority_list, format!("{strategy_name}.json"));
}

fn user_input_to_priority_list() -> Vec<strategy::NameCondition> {
    let mut input_list = Vec::<strategy::NameCondition>::new();

    println!("Enter items (type 'done' to finish):");

    'name_condition: loop {
        // In a loop in case of invalid card names
        let mut name = String::new(); // Declare input here

        'card_name: loop {
            println!("Enter card name (type 'done' to finish)");
            std::io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            name = name.trim().to_string().to_lowercase(); // Shadow the input variable

            if name == "done" {
                break 'name_condition;
            }

            if crate::card::constants::is_card(&name) {
                break 'card_name;
            } else {
                println!("Invalid card name. Please try again.");
            }
        }

        let condition = user_input_to_condition();
        let name_condition: strategy::NameCondition = strategy::NameCondition {
            card: name,
            condition,
        };
        input_list.push(name_condition);
    }

    input_list
}

fn user_input_to_condition() -> strategy::Condition {
    // Prompt for the first value
    let first = solicit_value();

    let condition_type = solicit_condition_type();

    let second = solicit_value();

    strategy::Condition {
        condition_type,
        first,
        second,
    }
}

fn solicit_value() -> strategy::ConditionValue {
    loop {
        let mut input = String::new();
        println!("Select a type of value for the first value to compare:");
        println!("1: Int");
        println!("2: Count in Deck");
        println!("3: Count Type in Deck");
        println!("4: Count in Supply");
        println!("5: Count All Cards in Deck");
        println!("6: Count VP");
        println!("7: Count Opponent's VP");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        let value = match trimmed_input {
            "1" => strategy::ConditionValue::Int(input_value("first value")),
            "2" => strategy::ConditionValue::CountInDeck(input_card_name("card name")),
            "3" => strategy::ConditionValue::CountTypeInDeck(card_type_input("card type")),
            "4" => strategy::ConditionValue::CountInSupply(input_card_name("card name")),
            "5" => strategy::ConditionValue::CountAllCardsInDeck,
            "6" => strategy::ConditionValue::CountVp,
            "7" => strategy::ConditionValue::CountOpponentVp,
            _ => {
                println!("Invalid option. Please select a valid option.");
                continue;
            }
        };

        return value;
    }
}

fn solicit_condition_type() -> strategy::ConditionType {
    loop {
        let mut input = String::new();
        println!("Select a type of value");
        println!("1: Greater Than");
        println!("2: Greater Than or Equal to");
        println!("3: Less Than");
        println!("4: Less Than or Equal to ");
        println!("5: Equal To");
        println!("6: Not Equal To");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        let value = match trimmed_input {
            "1" => strategy::ConditionType::GreaterThan,
            "2" => strategy::ConditionType::GreaterThanOrEqualTo,
            "3" => strategy::ConditionType::LessThan,
            "4" => strategy::ConditionType::LessThanOrEqualTo,
            "5" => strategy::ConditionType::EqualTo,
            "6" => strategy::ConditionType::NotEqualTo,
            _ => {
                println!("Invalid option. Please select a valid option.");
                continue;
            }
        };

        return value;
    }
}

fn input_value(prompt: &str) -> u16 {
    loop {
        let mut input = String::new();
        println!("Enter {}:", prompt);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid value.");
                continue;
            }
        }
    }
}

fn input_card_name(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("Enter {}:", prompt);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<String>() {
            Ok(card_name) => match card::constants::is_card(&card_name) {
                true => return card_name,
                false => {
                    println!("Not a card. Please enter a valid card name.")
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid value.");
                continue;
            }
        }
    }
}

fn card_type_input(prompt: &str) -> card::CardType {
    loop {
        let mut input = String::new();
        println!("Enter {}:", prompt);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        match trimmed_input {
            "Action" => return card::CardType::Action(Default::default()),
            "Treasure" => return card::CardType::Treasure(Default::default()),
            "Victory" => return card::CardType::Victory(Default::default()),
            _ => {
                println!("Invalid card type. Please enter a valid card type (Action, Treasure, Victory).");
                continue;
            }
        }
    }
}
