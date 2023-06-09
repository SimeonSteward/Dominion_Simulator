use crate::card::{Card, constants::*};
use crate::supply_pile::SupplyPile;
use std::collections::HashMap;

pub struct Kingdom<'a> {
    pub supply_piles: HashMap<&'a Card, SupplyPile<'a>>,
}

impl<'a> Kingdom<'a> {
    pub fn new() -> Self {
        Kingdom {
            supply_piles: HashMap::new(),
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
        add_supply_pile!(self, COPPER, 60);
        add_supply_pile!(self, SILVER, 40);
        add_supply_pile!(self, GOLD, 30);
        add_supply_pile!(self, ESTATE, 8);
        add_supply_pile!(self, DUCHY, 8);
        add_supply_pile!(self, PROVINCE, 8);
    }

    pub fn remove_from_supply(&mut self, card: &'a Card, n: u8) {
        if let Some(supply_pile) = self.supply_piles.get_mut(card) {
            supply_pile.count -= n;
        }
    }
}
