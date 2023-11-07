use crate::card::Card;
pub struct SupplyPile {
    pub card: &'static Card,
    pub count: u16,
}

impl SupplyPile {
    pub fn new(card: &'static Card, count: u16) -> Self {
        SupplyPile { card, count }
    }
}
