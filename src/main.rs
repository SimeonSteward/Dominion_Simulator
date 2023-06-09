mod card;
mod player;
mod utils;
mod kingdom;
mod supply_pile;
mod strategy;

use player::Player;
use kingdom::Kingdom;

fn main() {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    utils::print_kingdom(&kingdom);

    let mut player = Player::new("P1"); //
    player.initialize(&mut kingdom);
    player.cleanup();
    player.cleanup();
    utils::print_kingdom(&kingdom);
}
