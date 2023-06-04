use crate::card::Card;
pub struct SupplyPile<'a> {
    pub card: &'a Card,
    pub count: u8,
}

impl<'a> SupplyPile<'a> {
    pub fn new(card: &'a Card, count: u8) -> Self {
        SupplyPile { card, count }
    }
}
