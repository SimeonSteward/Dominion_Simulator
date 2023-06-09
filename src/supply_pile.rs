use crate::card::Card;
pub struct SupplyPile<'a> {
    pub card: &'a Card,
    pub count: u16,
}

impl<'a> SupplyPile<'a> {
    pub fn new(card: &'a Card, count: u16) -> Self {
        SupplyPile { card, count }
    }
}

