use crate::card::{COPPER, SILVER, GOLD, ESTATE, DUCHY, PROVINCE};
use crate::supply_pile::SupplyPile;

pub struct Kingdom<'a> {
    pub supply_piles: Vec<SupplyPile<'a>>,
}

impl<'a> Kingdom<'a> {
    pub fn new() -> Self {
        Kingdom {
            supply_piles: Vec::new(),
        }
    }

    pub fn initialize(&'a mut self) {
        macro_rules! add_supply_pile {
            ($self:expr, $card:expr, $count:expr) => {
                $self.supply_piles.push(SupplyPile::new(&$card, $count));
            };
        }
        add_supply_pile!(self, COPPER, 60);
        add_supply_pile!(self, SILVER, 40);
        add_supply_pile!(self, GOLD, 30);
        add_supply_pile!(self, ESTATE, 8);
        add_supply_pile!(self, DUCHY, 8);
        add_supply_pile!(self, PROVINCE, 8);
    }
}
