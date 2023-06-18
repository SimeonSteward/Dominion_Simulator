mod card;
mod kingdom;
mod player;
mod strategy;
mod supply_pile;
mod utils;

use kingdom::{GameOver, Kingdom};
use player::Player;
enum GameResult {
    Win,
    Tie,
    Loss,
}
fn run_game() -> GameResult {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    let mut player_1 = Player::new("Woodcutter"); //
    let mut player_2 = Player::new("Adventurer"); //
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
        return GameResult::Tie;
    } else {
        let winner;
        if player_1_vp > player_2_vp {
            winner = &player_1;
            println!(
                "{}: {}, {}: {}    {} wins",
                player_1.name,
                player_1.get_vp(),
                player_2.name,
                player_2.get_vp(),
                winner.name
            );
            return GameResult::Win;
        } else {
            winner = &player_2;
            println!(
                "{}: {}, {}: {}    {} wins",
                player_1.name,
                player_1.get_vp(),
                player_2.name,
                player_2.get_vp(),
                winner.name
            );
            return GameResult::Loss;
        };
    }
}
fn main() {
    let mut wins: u16 = 0;
    let mut ties: u16 = 0; 
    let mut losses: u16 = 0;
    for _ in 1..100 {
        
        match run_game(){
            GameResult::Win => { wins += 1;}
            GameResult::Tie => { ties += 1;}
            GameResult::Loss => { losses += 1;}
        }
        println!("Wins: {}, Losses: {}, Ties: {}", wins, losses, ties);
    }
}
