use crate::card::{constants::*, Card, CardType};
use crate::kingdom::Kingdom;
use crate::strategy::{CardCondition, BUY_PRIORITY, TREASURE_PLAY_PRIORITY_LIST};
use crate::utils::CardCollectionsTrait;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Player<'a> {
    pub deck: Vec<&'a Card>,
    pub hand: HashMap<&'a Card, u16>,
    pub discard: Vec<&'a Card>,
    pub cards: HashMap<&'a Card, u16>,
    pub name: &'static str,
    pub abreviated_name: &'static str,
    pub turn_number: u16,
    pub actions: u16,
    pub buys: u16,
    pub coins: u16,
}

impl<'a> Player<'a> {
    pub fn new(name: &'static str) -> Self {
        Player {
            deck: Vec::new(),
            hand: HashMap::new(),
            discard: Vec::new(),
            cards: HashMap::new(),
            name,
            abreviated_name: &name[..2],
            turn_number: 0,
            actions: 0,
            buys: 0,
            coins: 0,
        }
    }

    pub fn initialize(&mut self, kingdom: &mut Kingdom) {
        kingdom.remove_from_supply(&COPPER, 7);
        self.add_to_discard(&COPPER, 7);
        println!("{} starts with 7 Coppers", self.abreviated_name);
        self.add_to_discard(&ESTATE, 3);
        println!("{} starts with 3 Estates", self.abreviated_name);
        self.cleanup();
        self.actions = 1;
        self.buys = 1;
        self.coins = 0;
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
        print!("{} Draws: ", self.abreviated_name);
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
        println!("{} shuffles their deck... ", self.abreviated_name);
        self.deck.extend(self.discard.drain(..));
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn add_to_discard(&mut self, card: &'a Card, n: u16) {
        for _ in 0..n {
            self.discard.push(card);
        }
        let count = self.cards.entry(card).or_insert(0);
        *count += n;
    }

    pub fn gain(&mut self, kingdom: &mut Kingdom<'a>, card: &'a Card, n: u16) {
        kingdom.remove_from_supply(card, n);
        self.add_to_discard(card, n);
        // print!("{} Gains: {} {}", self.name, n, card.name);
    }

    pub fn turn(&mut self, kingdom: &mut Kingdom<'a>) {
        self.turn_number += 1;
        println!("\nTurn {} - {}", self.turn_number, self.name);
        self.action_phase();
        self.buy_phase(kingdom);
        self.cleanup();
    }

    pub fn action_phase(&mut self) {}
    pub fn buy_phase(&mut self, kingdom: &mut Kingdom<'a>) {
        self.play_treasures();
        self.purchase_phase(kingdom);
    }

    pub fn play_treasures(&mut self) {
        for treasure_play_priority in TREASURE_PLAY_PRIORITY_LIST.iter() {
            let card = treasure_play_priority.card;

            let card_from_hand = self.hand.entry(card);
            match card_from_hand {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    let num_card_in_hand = *occupied.get();
                    match &treasure_play_priority.condition {
                        Some(condition) => {
                            if condition(self) {
                                self.play_treasure(card, num_card_in_hand);
                            }
                        }
                        None => {
                            self.play_treasure(card, num_card_in_hand);
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {}
            }
        }
    }

    pub fn play_treasure(&mut self, card: &'a Card, n: u16) {
        match card.card_type {
            CardType::Treasure { coin } => {
                self.coins += coin * n;
                println!("{} plays {} {}s", self.name, n, card.name);
                self.add_to_discard(card, n);
                self.hand.remove(card); //TODO What if n is not all of the treasure
            }
            _ => {
                println!("ERROR: Tried to play {} as treasure", card.name)
            }
        }
    }

    pub fn purchase_phase(&mut self, kingdom: &mut Kingdom<'a>) {
        while self.buys > 0 {
            for buy_priority in BUY_PRIORITY.iter() {
                if buy_priority.card.cost <= self.coins
                    && kingdom
                        .supply_piles
                        .get(buy_priority.card)
                        .and_then(|supply_pile| Some(supply_pile.count > 0))
                        .unwrap_or(false)
                {
                    match &buy_priority.condition {
                        Some(condition) => {
                            if condition(self) {
                                self.buy_card(kingdom, buy_priority);
                                break;
                            }
                        }
                        None => {
                            self.buy_card(kingdom, buy_priority);
                            break;
                        }
                    }
                }
            }
            break; // If player goes through the entire buy priority list and doesn't want to buy anything, purchasing is over
        }
    }

    fn buy_card(&mut self, kingdom: &mut Kingdom<'a>, buy_priority: &CardCondition<'a>) {
        self.coins -= buy_priority.card.cost;
        self.buys -= 1;
        self.gain(kingdom, buy_priority.card, 1);
        println!("{} buys and gains a {}", self.name, buy_priority.card.name);
    }

    pub fn cleanup(&mut self) {
        for (card, count) in self.hand.drain() {
            for _ in 0..count {
                self.discard.push(card);
            }
        }
        self.draw(5);
        self.actions = 1;
        self.buys = 1;
        self.coins = 0;
    }
}
