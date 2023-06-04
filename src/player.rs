use crate::card::{Card, COPPER, ESTATE};
use crate::utils::print_cards;
use rand::seq::SliceRandom;

pub struct Player<'a> {
    pub deck: Vec<&'a Card>,
    pub hand: Vec<&'a Card>,
    pub discard: Vec<&'a Card>,
    pub name: &'static str,
}

impl<'a> Player<'a> {
    pub fn new(name: &'static str) -> Self {
        Player {
            deck: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
            name,

        }
    }

    pub fn initialize(&mut self) {
        self.add_to_discard(7, &COPPER);
        self.add_to_discard(3, &ESTATE);
        self.cleanup();
    }

    pub fn draw(&mut self, n: usize) {
        if self.deck.len() >= n {
            self.hand.extend(self.deck.drain(..n));
        } else {
            let remaining_cards = self.deck.len();
            self.hand.extend(self.deck.drain(..remaining_cards));
            self.shuffle();
            let additional_cards_needed = n - remaining_cards;
            self.hand.extend(self.deck.drain(..additional_cards_needed));
        }
        print!("{} Draws: ", self.name);
        print_cards(&self.hand);
        println!();
    }

    pub fn shuffle(&mut self) {
        println!("{} Shuffles... ", self.name);
        self.deck.extend(self.discard.drain(..));
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn add_to_discard(&mut self, n: usize, card: &'a Card) {
        for _ in 0..n {
            self.discard.push(card);
        }
    }

    pub fn cleanup(&mut self) {
        self.discard.extend(self.hand.drain(..));
        self.draw(5);
    }
}
