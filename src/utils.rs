use crate::card::Card;
use crate::kingdom::Kingdom;

pub fn print_cards(cards: &Vec<&Card>) {
    for card in cards {
        print!("{}, ", card.name);
    }
}

pub fn print_kingdom(kingdom: &Kingdom) {
    for supply_pile in &kingdom.supply_piles {
        println!("Card: {}, Count: {}", &supply_pile.1.card.name, supply_pile.1.count);
    }
}

