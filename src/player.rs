
use crate::card::Card;
use rand::seq::SliceRandom;

pub struct Player<'a> {
    pub deck: Vec<&'a Card>,
    pub hand: Vec<&'a Card>,
    pub discard: Vec<&'a Card>,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        Player {
            deck: Vec::new(),
            hand: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn draw(&mut self, n: usize) {
        if self.deck.len() > n {
            self.hand.extend(self.deck.drain(..n));
        } else {
            let i: usize = n - self.deck.len();
            self.hand.extend(self.deck.drain(..self.deck.len()));
            self.shuffle();
            self.hand.extend(self.deck.drain(..i));
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.extend(self.discard.drain(..));
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn gain(&mut self, n: usize, card: &'a Card) {
        for _ in 0..n {
            self.discard.push(card);
        }
    }

    pub fn cleanup(&mut self) {
        self.discard.extend(self.hand.drain(..));
        self.draw(5);
    }
}
