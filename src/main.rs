mod card;
mod kingdom;
mod player;
mod strategy;
mod supply_pile;
mod utils;
pub mod cli;
use kingdom::{GameOver, Kingdom};
use player::Player;
// use std::sync::atomic::AtomicUsize;
// use std::sync::{Arc, Mutex};
// use std::thread;
use std::time::Instant;
use strategy::CardCondition;
enum GameResult {
    Win,
    Tie,
    Loss,
}
fn run_game(
    print_log: bool,
    p1_buy_priority: &Vec<CardCondition>,
    p1_action_play_priority_list: &Vec<CardCondition<'_>>,
    p1_treasure_play_priority_list: &Vec<CardCondition<'_>>,
    p2_buy_priority: &Vec<CardCondition>,
    p2_action_play_priority_list: &Vec<CardCondition<'_>>,
    p2_treasure_play_priority_list: &Vec<CardCondition<'_>>,
) -> GameResult {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    let mut player_1 = Player::new(
        "Woodcutter",
        false,
        p1_buy_priority,
        p1_action_play_priority_list,
        p1_treasure_play_priority_list,
    ); //
    let mut player_2 = Player::new(
        "Adventurer",
        false,
        p2_buy_priority,
        p2_action_play_priority_list,
        p2_treasure_play_priority_list,
    ); //
    player_1.initialize(&mut kingdom);
    player_2.initialize(&mut kingdom);
    while kingdom.game_end != GameOver::IsOver {
        player_1.turn(&player_2, &mut kingdom);
        if kingdom.game_end == GameOver::IsOver {
            break;
        }
        player_2.turn(&player_1, &mut kingdom);
    }

    let player_1_vp = player_1.get_vp();
    let player_2_vp = player_2.get_vp();
    let player_1_win: GameResult;
    if player_1_vp == player_2_vp && player_1.turn_number == player_2.turn_number {
        player_1_win = GameResult::Tie;
    } else if player_1_vp > player_2_vp {
        player_1_win = GameResult::Win;
    } else {
        player_1_win = GameResult::Loss;
    }
    if print_log {
        let verb = match player_1_win {
            GameResult::Win => "wins agaist",
            GameResult::Tie => "ties with",
            GameResult::Loss => "loses to",
        };
        println!(
            "{}: {} {} {}: {}",
            player_1.name,
            player_1.get_vp(),
            verb,
            player_2.name,
            player_2.get_vp(),
        );
    }

    player_1_win
}

fn single_treaded(
    p1_buy_priority: Vec<CardCondition>,
    p1_action_play_priority_list: Vec<CardCondition<'_>>,
    p1_treasure_play_priority_list: Vec<CardCondition<'_>>,
    p2_action_play_priority_list: Vec<CardCondition<'_>>,
    p2_treasure_play_priority_list: Vec<CardCondition<'_>>,

    p2_buy_priority: Vec<CardCondition>,
) {
    let start_time = Instant::now();

    let mut wins: u16 = 0;
    let mut ties: u16 = 0;
    let mut losses: u16 = 0;

    for _ in 1..10000 {
        match run_game(
            false,
            &p1_buy_priority,
            &p1_action_play_priority_list,
            &p1_treasure_play_priority_list,
            &p2_buy_priority,
            &p2_action_play_priority_list,
            &p2_treasure_play_priority_list,
        ) {
            GameResult::Win => {
                wins += 1;
            }
            GameResult::Tie => {
                ties += 1;
            }
            GameResult::Loss => {
                losses += 1;
            }
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Wins: {}, Losses: {}, Ties: {}", wins, losses, ties);
    println!("Elapsed time: {:?}", elapsed_time);
}
/*
fn multi_threaded() {
    let wins = Arc::new(Mutex::new(0));
    let ties = Arc::new(Mutex::new(0));
    let losses = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    let start_time = Instant::now();
    for _ in 1..10000 {
        let wins = Arc::clone(&wins);
        let ties = Arc::clone(&ties);
        let losses = Arc::clone(&losses);

        let handle = thread::spawn(move || match run_game(false) {
            GameResult::Win => {
                *wins.lock().unwrap() += 1;
            }
            GameResult::Tie => {
                *ties.lock().unwrap() += 1;
            }
            GameResult::Loss => {
                *losses.lock().unwrap() += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Wins: {}, Losses: {}, Ties: {}",
        *wins.lock().unwrap(),
        *losses.lock().unwrap(),
        *ties.lock().unwrap()
    );
    println!("Elapsed time: {:?}", elapsed_time);
}
*/
/*
#[tokio::main]
async fn multi_threaded_tokio() {
    let wins = Arc::new(AtomicUsize::new(0));
    let ties = Arc::new(AtomicUsize::new(0));
    let losses = Arc::new(AtomicUsize::new(0));

    let mut tasks = vec![];

    let start_time = Instant::now();
    for _ in 1..10000 {
        let wins = Arc::clone(&wins);
        let ties = Arc::clone(&ties);
        let losses = Arc::clone(&losses);

        let task = tokio::task::spawn(async move {
            match run_game(false) {
                GameResult::Win => {
                    wins.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                GameResult::Tie => {
                    ties.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                GameResult::Loss => {
                    losses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Wins: {}, Losses: {}, Ties: {}",
        wins.load(std::sync::atomic::Ordering::Relaxed),
        losses.load(std::sync::atomic::Ordering::Relaxed),
        ties.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("Elapsed time: {:?}", elapsed_time);
}
*/


fn main() {
    // single_treaded();
    // multi_threaded();
    // multi_threaded_tokio();
    cli::create_new_priority();
}
