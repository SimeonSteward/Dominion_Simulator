mod card;

use card::{Card, COPPER, ESTATE};
use rand::seq::SliceRandom;

fn draw<'a>(
    n: usize,
    deck: &mut Vec<&'a Card>,
    hand: &mut Vec<&'a Card>,
    discard: &mut Vec<&'a Card>,
) {
    if deck.len() > n {
        hand.extend(deck.drain(..n));
    } else {
        let i: usize = n - deck.len();
        hand.extend(deck.drain(..deck.len()));
        shuffle(deck, discard);
        hand.extend(deck.drain(..i));
    }
}

fn shuffle<'a>(deck: &mut Vec<&'a Card>, discard: &mut Vec<&'a Card>) {
    deck.extend(discard.drain(..));
    deck.shuffle(&mut rand::thread_rng());
}

fn gain<'a>(n: usize, card: &'a Card, discard: &mut Vec<&'a Card>) {
    for _ in 0..n {
        discard.push(card);
    }
}

fn cleanup<'a>(deck: &mut Vec<&'a Card>, hand: &mut Vec<&'a Card>, discard: &mut Vec<&'a Card>) {
    discard.extend(hand.drain(..));
    draw(5, deck, hand, discard);
}

fn main() {
    let copper: &Card = &*COPPER;
    let estate: &Card = &*ESTATE;

    let mut deck: Vec<&Card> = vec![];
    let mut hand: Vec<&Card> = vec![];
    let mut discard: Vec<&Card> = vec![];

    gain(7, copper, &mut discard);
    gain(3, estate, &mut discard);

    cleanup(&mut deck, &mut hand, &mut discard);

    for card in hand {
        print!("{}, ", card.name);
    }
}
