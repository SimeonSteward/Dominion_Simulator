mod card;
mod kingdom;
mod player;
mod strategy;
mod supply_pile;
mod utils;

use kingdom::{GameOver, Kingdom};
use player::Player;

fn main() {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    let mut player_1 = Player::new("Player 1"); //
    let mut player_2 = Player::new("Player 2"); //
    player_1.initialize(&mut kingdom);
    player_2.initialize(&mut kingdom);
    while kingdom.game_end != GameOver::IsOver {
        player_1.turn(&mut kingdom);
        if kingdom.game_end == GameOver::IsOver {
            break;
        }
        player_2.turn(&mut kingdom);
    }

    let player_1_vp = player_1.get_vp();
    let player_2_vp = player_2.get_vp();
    if player_1_vp == player_2_vp && player_1.turn_number == player_2.turn_number {
        println!(
            "{}: {}, {}: {}  {} and {} tie",
            player_1.name,
            player_1.get_vp(),
            player_2.name,
            player_2.get_vp(),
            player_1.name,
            player_2.name,
        );
    } else {
        let winner;
        if player_1_vp > player_2_vp {
            winner = &player_1;
        } else {
            winner = &player_2
        };

        println!(
            "{}: {}, {}: {}    {} wins",
            player_1.name,
            player_1.get_vp(),
            player_2.name,
            player_2.get_vp(),
            winner.name
        );
    }
}
