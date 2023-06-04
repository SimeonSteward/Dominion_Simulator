mod card;
mod player;
mod utils;
mod kingdom;
mod supply_pile;

use player::Player;
use kingdom::Kingdom;

fn main() {
    let mut kingdom = Kingdom::new();
    let mut player = Player::new("P1"); //
    kingdom.initialize();
    player.initialize();
    player.cleanup();
    player.cleanup();
}
