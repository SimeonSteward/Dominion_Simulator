use crate::card::{constants::*, Card, CardType};
use crate::kingdom::Kingdom;
use crate::strategy::CardCondition;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct PlayerPriorities<'a> {
    pub buy_priority: &'a Vec<CardCondition<'a>>,
    pub action_play_priority: &'a Vec<CardCondition<'a>>,
    pub treasure_play_priority: &'a Vec<CardCondition<'a>>,
}
pub struct Player<'a> {
    pub print_log: bool,
    pub deck: Vec<&'a Card>,
    pub cards_in_play: Vec<&'a Card>,
    pub discard: Vec<&'a Card>,
    pub hand: HashMap<&'a Card, u16>,
    pub cards: HashMap<&'a Card, u16>,
    pub name: &'static str,
    pub abreviated_name: &'static str,
    pub turn_number: u16,
    pub actions: u16,
    pub buys: u16,
    pub coins: u16,
    pub vp_tokens: u16,
    pub player_priorities: &'a PlayerPriorities<'a>,
}

impl<'a> Player<'a> {
    pub fn new(
        name: &'static str,
        print_log: bool,
        player_priorities: &'a PlayerPriorities<'a>,
    ) -> Self {
        Player {
            print_log,
            deck: Vec::new(),
            cards_in_play: Vec::new(),
            discard: Vec::new(),
            hand: HashMap::new(),
            cards: HashMap::new(),
            name,
            abreviated_name: &name[..2],
            turn_number: 0,
            actions: 0,
            buys: 0,
            coins: 0,
            vp_tokens: 0,
            player_priorities,
        }
    }

    pub fn initialize(&mut self, kingdom: &mut Kingdom) {
        kingdom.remove_from_supply(&supply::copper::COPPER, 7);
        self.add_to_discard(&supply::copper::COPPER, 7);
        self.add_to_discard(&supply::estate::ESTATE, 3);
        if self.print_log {
            println!("{} starts with 7 Coppers", self.abreviated_name);
            println!("{} starts with 3 Estates", self.abreviated_name);
        }
        self.cleanup();
    }

    pub fn draw(&mut self, n: u16) {
        if self.deck.len() as u16 >= n {
            self.add_cards_to_hand(n);
        } else {
            let remaining_cards = self.deck.len() as u16;
            self.add_cards_to_hand(remaining_cards);
            self.shuffle();
            let additional_cards_needed = std::cmp::min(n - remaining_cards, self.deck.len().try_into().expect("Failed to convert usize into u16"));
            self.add_cards_to_hand(additional_cards_needed);
        }
        if self.print_log {
            println!();
        }
    }

    fn add_cards_to_hand(&mut self, n: u16) {
        if n == 0 {
            return;
        }
        if self.print_log {
            print!("{} Draws: ", self.abreviated_name);
        }
        for card in self.deck.drain(..n as usize) {
            if self.print_log {
                print!("{}, ", card.name);
            }
            let count = self.hand.entry(card).or_insert(0);
            *count += 1;
        }
    }

    pub fn shuffle(&mut self) {
        if self.print_log {
            println!("{} shuffles their deck... ", self.abreviated_name);
        }
        self.deck.append(&mut self.discard);
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

    pub fn turn(&mut self, opponent: &mut Player, kingdom: &mut Kingdom<'a>) {
        self.turn_number += 1;
        if self.print_log {
            println!("\nTurn {} - {}", self.turn_number, self.name);
        }
        self.action_phase(opponent, kingdom);
        self.buy_phase(opponent, kingdom);
        self.cleanup();
    }

    pub fn action_phase(&mut self, opponent: &mut Player, kingdom: &Kingdom) {
        'actions_left: while self.actions >= 1 {
            for action_play_priority in self.player_priorities.action_play_priority {
                let card = action_play_priority.card;
                let card_from_hand = self.hand.entry(card);
                match card_from_hand {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        if *entry.get() >= 1
                            && (action_play_priority.condition)(self, opponent, kingdom)
                        {
                            self.play_action_from_hand(card, opponent);
                            continue 'actions_left;
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(_) => {}
                }
            }
            break;
        }
    }

    pub fn buy_phase(&mut self, opponent: &Player, kingdom: &mut Kingdom<'a>) {
        self.play_treasures(opponent, kingdom);
        self.purchase_phase(opponent, kingdom);
    }

    pub fn play_treasures(&mut self, opponent: &Player, kingdom: &Kingdom) {
        for treasure_play_priority in self.player_priorities.treasure_play_priority {
            let card = treasure_play_priority.card;

            let card_from_hand = self.hand.entry(card);
            match card_from_hand {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    let num_card_in_hand = *occupied.get();
                    if (treasure_play_priority.condition)(self, opponent, kingdom) {
                        self.play_treasure_from_hand(card, num_card_in_hand);
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {}
            }
        }
    }

    pub fn play_treasure_from_hand(&mut self, card: &'a Card, n: u16) {
        match &card.card_type {
            CardType::Treasure => {
                if self.print_log {
                    println!("{} plays {} {}s", self.name, n, card.name);
                }
                (card.play_treasure)(self, n);
                for _ in 0..n {
                    self.cards_in_play.push(card);
                }
                match self.hand.entry(card) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let num_in_hand = *entry.get_mut();
                        if num_in_hand >= n {
                            *entry.get_mut() -= n;
                        } else {
                            eprintln!("ERROR: Tried to play {} {}s when not in hand", n, card.name);
                            std::process::exit(1);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(_) => {
                        eprintln!("ERROR: Tried to play {} when not in hand", card.name);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("ERROR: Tried to play {} as treasure", card.name)
            }
        }
    }

    pub fn play_action_from_hand(&mut self, card: &'a Card, opponent: &mut Player) {
        if self.actions >= 1 {
            match &card.card_type {
                CardType::Action => {
                    if self.print_log {
                        println!("{} plays a {}", self.name, card.name);
                    }
                    (card.play_action)(self, opponent);
                    self.actions -= 1;
                    self.cards_in_play.push(card);

                    //Removes card from hand
                    match self.hand.entry(card) {
                        std::collections::hash_map::Entry::Occupied(mut entry) => {
                            let num_in_hand = *entry.get_mut();
                            if num_in_hand >= 1 {
                                *entry.get_mut() -= 1;
                            } else {
                                eprintln!("ERROR: Tried to play {} when not in hand", card.name);
                                std::process::exit(1);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(_) => {
                            eprintln!("ERROR: Tried to play {} when not in hand", card.name);
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    eprintln!("ERROR: Tried to play {} as action", card.name);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("ERROR: Tried to play {} when out of actions", card.name);
            std::process::exit(1);
        }
    }

    pub fn purchase_phase(&mut self, opponent: &Player, kingdom: &mut Kingdom<'a>) {
        let mut done = false;
        while self.buys > 0 && !done {
            done = true;
            'priority: for buy_priority in self.player_priorities.buy_priority {
                if buy_priority.card.cost <= self.coins
                    && kingdom
                        .supply_piles
                        .get(buy_priority.card)
                        .map(|supply_pile| supply_pile.count > 0)
                        .unwrap_or(false)
                    && (buy_priority.condition)(self, opponent, kingdom)
                {
                    self.buy_card(kingdom, buy_priority.card);
                    done = false;
                    break 'priority;
                }
            }
        }
    }

    fn buy_card(&mut self, kingdom: &mut Kingdom<'a>, card: &'a Card) {
        self.coins -= card.cost;
        self.buys -= 1;
        self.gain(kingdom, card, 1);
        if self.print_log {
            println!("{} buys and gains a {}", self.name, card.name);
        }
    }

    pub fn cleanup(&mut self) {
        for (card, count) in self.hand.drain() {
            for _ in 0..count {
                self.discard.push(card);
            }
        }
        self.discard.append(&mut self.cards_in_play);
        self.draw(5);
        self.actions = 1;
        self.buys = 1;
        self.coins = 0;
    }

    pub fn get_vp(&self) -> u16 {
        let mut total_vp: u16 = self.vp_tokens;
        for (card, quantity) in self.cards.iter() {
            if let CardType::Victory = &card.card_type {
                let vp = (card.points)(self);
                total_vp += vp * quantity;
            }
        }
        total_vp
    }

    pub fn get_coin(&self) -> u16 {
        let mut total_coin: u16 = 0;
        for (card, quantity) in self.cards.iter() {
            if let CardType::Treasure = &card.card_type {
                total_coin += card.coin * quantity;
            }
        }
        total_coin
    }
}
