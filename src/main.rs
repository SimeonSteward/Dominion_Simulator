mod card;
mod player;
mod utils;

use player::Player;

fn main() {
    let mut player = Player::new("P1"); //
    player.initialize();
    player.cleanup();
    player.cleanup();
}
