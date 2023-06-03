mod card;
mod player;

use player::Player;

fn main() {
    let copper = &*card::COPPER;
    let estate = &*card::ESTATE;

    let mut player = Player::new();

    player.gain(7, copper);
    player.gain(3, estate);

    player.cleanup();

    for card in &player.hand {
        print!("{}, ", card.name);
    }
}
