use core::{
    kingdom,
    player::{self, PlayerPriorities},
};
use kingdom::{GameOver, Kingdom};
use player::Player;
use std::{
    cmp::Ordering,
    sync::{atomic::AtomicUsize, Arc},
    time::Instant,
};
enum GameResult {
    Win,
    Tie,
    Loss,
}

fn run_game(
    print_log: bool,
    p1_is_first_player: bool,
    p1_priorities: &PlayerPriorities,
    p2_priorities: &PlayerPriorities,
) -> GameResult {
    let mut kingdom = Kingdom::new();
    kingdom.initialize();
    let mut player_1 = Player::new("Woodcutter", print_log, p1_priorities); //
    let mut player_2 = Player::new("Adventurer", print_log, p2_priorities); //
    player_1.initialize(&mut kingdom);
    player_2.initialize(&mut kingdom);
    if !p1_is_first_player {
        player_2.turn(&player_1, &mut kingdom);
    }
    while kingdom.game_end != GameOver::IsOver {
        player_1.turn(&player_2, &mut kingdom);
        if kingdom.game_end == GameOver::IsOver {
            break;
        }
        player_2.turn(&player_1, &mut kingdom);
    }

    let player_1_vp = player_1.get_vp();
    let player_2_vp = player_2.get_vp();
    let player_1_win: GameResult = match player_1_vp.cmp(&player_2_vp) {
        Ordering::Less => GameResult::Loss,
        Ordering::Equal => {
            match (
                p1_is_first_player,
                player_1.turn_number == player_2.turn_number,
            ) {
                (true, true) => GameResult::Tie,
                (true, false) => GameResult::Loss,
                (false, true) => GameResult::Tie,
                (false, false) => GameResult::Win,
            }
        }
        Ordering::Greater => GameResult::Win,
    };

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

// fn single_treaded(
//     num_trials: u32,
//     p1_priorities: &PlayerPriorities,
//     p2_priorities: &PlayerPriorities,
// ) {
//     let start_time = Instant::now();

//     let mut wins: u32 = 0;
//     let mut ties: u32 = 0;
//     let mut losses: u32 = 0;
//     let mut p1_is_first_player: bool = true;

//     for _ in 0..num_trials {
//         match run_game(false, p1_is_first_player, p1_priorities, p2_priorities) {
//             GameResult::Win => {
//                 wins += 1;
//             }
//             GameResult::Tie => {
//                 ties += 1;
//             }
//             GameResult::Loss => {
//                 losses += 1;
//             }
//         }
//         p1_is_first_player = !p1_is_first_player;
//     }
//     let elapsed_time = start_time.elapsed();
//     println!("Wins: {}, Losses: {}, Ties: {}", wins, losses, ties);
//     println!("Elapsed time: {:?}", elapsed_time);
// }
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
#[tokio::main]
async fn multi_threaded_tokio(
    num_trials: u32,
    p1_priorities: &PlayerPriorities,
    p2_priorities: &PlayerPriorities,
) {
    let wins = Arc::new(AtomicUsize::new(0));
    let ties = Arc::new(AtomicUsize::new(0));
    let losses = Arc::new(AtomicUsize::new(0));


    let start_time = Instant::now();

    tokio_scoped::scope(|scope| {
        for index in 0..num_trials {
            let wins = Arc::clone(&wins);
            let ties = Arc::clone(&ties);
            let losses = Arc::clone(&losses);

            scope.spawn(async move {
                match run_game(false, index % 2 != 0, p1_priorities, p2_priorities) {
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
        }
    });

    

    let elapsed_time = start_time.elapsed();
    println!(
        "Wins: {}, Losses: {}, Ties: {}",
        wins.load(std::sync::atomic::Ordering::Relaxed),
        losses.load(std::sync::atomic::Ordering::Relaxed),
        ties.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("Elapsed time: {:?}", elapsed_time);
}

fn main() {
    let action_play = core::strategy::get_priority_list("action_play_priority".to_owned())
        .expect("Error Loading Action Play priority:");
    let treasure_play = core::strategy::get_priority_list("treasure_play_priority".to_owned())
        .expect("Error Loading Treasure Play priority:");
    let big_money = core::strategy::get_priority_list("big_money_ultimate".to_owned())
        .expect("Error Loading Action Play priority:");
    let smithy_money = core::strategy::get_priority_list("smithy_money".to_owned())
        .expect("Error Loading Action Play priority:");
    let p1_priorities = PlayerPriorities {
        buy_priority: &big_money,
        action_play_priority: &action_play,
        treasure_play_priority: &treasure_play,
    };
    let p2_priorities = PlayerPriorities {
        buy_priority: &smithy_money,
        action_play_priority: &action_play,
        treasure_play_priority: &treasure_play,
    };

    //single_treaded(10000, &p1_priorities, &p2_priorities);
    // multi_threaded();
    multi_threaded_tokio(100000, &p1_priorities, &p2_priorities);
}
