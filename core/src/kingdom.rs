use crate::card::{constants::*, Card};
use crate::supply_pile::SupplyPile;
use std::collections::HashMap;

pub struct Kingdom<'kingdom> {
    pub supply_piles: HashMap<&'kingdom Card, SupplyPile>,
    pub game_end: GameOver,
    pub trash: HashMap<&'kingdom Card, u16>
}

#[derive(PartialEq)]
pub enum GameOver {
    IsOver,
    NotOver(u8),
}

impl <'a>Kingdom<'a>{
    pub fn new() -> Self {
        Kingdom {
            supply_piles: HashMap::new(),
            game_end: GameOver::NotOver(3),
            trash: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        macro_rules! add_supply_pile {
            ($self:expr, $card:expr, $count:expr) => {
                $self
                    .supply_piles
                    .insert(&$card, SupplyPile::new(&$card, $count));
            };
        }
        add_supply_pile!(self, supply::copper::COPPER, 60);
        add_supply_pile!(self, supply::silver::SILVER, 40);
        add_supply_pile!(self, supply::gold::GOLD, 30);
        add_supply_pile!(self, supply::estate::ESTATE, 8);
        add_supply_pile!(self, supply::duchy::DUCHY, 8);
        add_supply_pile!(self, supply::province::PROVINCE, 8);
        add_supply_pile!(self, base::smithy::SMITHY, 10);
        add_supply_pile!(self, base::village::VILLAGE, 10);
        add_supply_pile!(self, base::council_room::COUNCIL_ROOM, 10);
        add_supply_pile!(self, base::chapel::CHAPEL, 10);
    }

    pub fn remove_from_supply(&mut self, card: &'a Card, n: u16) {
        if let Some(supply_pile) = self.supply_piles.get_mut(card) {
            supply_pile.count -= n;
            if supply_pile.count == 0 {
                if *supply_pile.card == *supply::province::PROVINCE {
                    self.game_end = GameOver::IsOver;
                }
                match self.game_end {
                    GameOver::IsOver | GameOver::NotOver(1) => {
                        self.game_end = GameOver::IsOver;
                    }
                    GameOver::NotOver(num) if num > 1 => {
                        self.game_end = GameOver::NotOver(num - 1);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn add_card_to_trash(&mut self, card: &'a Card, n: u16){
       let count = self.trash.entry(card).or_insert(0);
        *count += n; 
    }
}
