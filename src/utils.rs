use crate::card::Card;

pub fn print_cards(cards: &Vec<&Card>) {
    for card in cards {
        print!("{}, ", card.name);
    }
}
