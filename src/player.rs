use crate::card::{constants::*, Card, CardType};
use crate::kingdom::Kingdom;
use crate::strategy::{CardCondition, TREASURE_PLAY_PRIORITY};
use crate::utils::CardCollectionsTrait;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Player<'a> {
    pub deck: Vec<&'a Card>,
    pub hand: HashMap<&'a Card, u8>,
    pub discard: Vec<&'a Card>,
    pub name: &'static str,
}

impl<'a> Player<'a> {
    pub fn new(name: &'static str) -> Self {
        Player {
            deck: Vec::new(),
            hand: HashMap::new(),
            discard: Vec::new(),
            name,
        }
    }

    pub fn initialize(&mut self, kingdom: &mut Kingdom) {
        kingdom.remove_from_supply(&COPPER, 7);
        self.add_to_discard(&COPPER, 7);
        self.add_to_discard(&ESTATE, 3);
        self.cleanup();
    }

    pub fn draw(&mut self, n: usize) {
        if self.deck.len() >= n {
            self.add_cards_to_hand(n);
        } else {
            let remaining_cards = self.deck.len();
            self.add_cards_to_hand(remaining_cards);
            self.shuffle();
            let additional_cards_needed = n - remaining_cards;
            self.add_cards_to_hand(additional_cards_needed);
        }
        print!("{} Draws: ", self.name);
        CardCollectionsTrait::print_cards(&self.hand);
        println!();
    }

    fn add_cards_to_hand(&mut self, n: usize) {
        for card in self.deck.drain(..n) {
            let count = self.hand.entry(card).or_insert(0);
            *count += 1;
        }
    }

    pub fn shuffle(&mut self) {
        println!("{} Shuffles... ", self.name);
        self.deck.extend(self.discard.drain(..));
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn add_to_discard(&mut self, card: &'a Card, n: u8) {
        for _ in 0..n {
            self.discard.push(card);
        }
    }

    pub fn gain(&mut self, n: u8, card: &'a Card, kingdom: &mut Kingdom<'a>) {
        kingdom.remove_from_supply(card, n);
        self.add_to_discard(card, n);
        print!("{} Gains: {} {}", self.name, n, card.name);
    }

    pub fn action_phase(&mut self) {}
    pub fn buy_phase(&mut self) {
        self.play_treasures();
    }

    pub fn play_treasures(&mut self) {
        for card_condition in TREASURE_PLAY_PRIORITY.iter() {
            let card = card_condition.card;

            let card_from_hand = self.hand.entry(card);
            match card_from_hand {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    let num_card_in_hand = *occupied.get();
                    match &card_condition.condition {
                        Some(condition) => {
                            if condition(self) {
                                self.play_treasure(card, num_card_in_hand);
                                self.add_to_discard(card, num_card_in_hand);
                                self.hand.remove(card);
                            }
                        }
                        None => {
                            self.play_treasure(card, num_card_in_hand);
                            self.add_to_discard(card, num_card_in_hand);
                            self.hand.remove(card);
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {}
            }
        }
    }

    pub fn play_treasure(&mut self, card: &Card, n: u8) {}

    pub fn cleanup(&mut self) {
        for (card, count) in self.hand.drain() {
            for _ in 0..count {
                self.discard.push(card);
            }
        }
        self.draw(5);
    }
}
