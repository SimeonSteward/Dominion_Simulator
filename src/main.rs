mod card;
mod kingdom;
mod player;
mod strategy;
mod supply_pile;
mod utils;

use card::constants::PROVINCE;
use kingdom::Kingdom;
use player::Player;

fn main() {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    let mut player_1 = Player::new("Player 1"); //
    let mut player_2 = Player::new("Player 2"); //
    player_1.initialize(&mut kingdom);
    player_2.initialize(&mut kingdom);
    while kingdom
        .supply_piles
        .get(&*PROVINCE)
        .map_or(false, |supply_pile| supply_pile.count > 0)
    {
        player_1.turn(&mut kingdom);
        player_2.turn(&mut kingdom);
    }

    utils::print_kingdom(&kingdom);
}
