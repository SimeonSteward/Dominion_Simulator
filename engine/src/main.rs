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
    let mut player_1 = Player::new(p1_priorities.name, print_log, p1_priorities); //
    let mut player_2 = Player::new(p2_priorities.name, print_log, p2_priorities); //
    player_1.initialize(&mut kingdom);
    player_2.initialize(&mut kingdom);
    if !p1_is_first_player {
        player_2.turn(&mut player_1, &mut kingdom);
    }
    while kingdom.game_end != GameOver::IsOver {
        player_1.turn(&mut player_2, &mut kingdom);
        if kingdom.game_end == GameOver::IsOver {
            break;
        }
        player_2.turn(&mut player_1, &mut kingdom);
    }

    let player_1_vp = player_1.get_vp();
    let player_2_vp = player_2.get_vp();
    let player_1_win: GameResult = match player_1_vp.cmp(&player_2_vp) {
        Ordering::Less => GameResult::Loss,
        Ordering::Equal => match player_1.turn_number.cmp(&player_2.turn_number) {
            Ordering::Less => GameResult::Win,
            Ordering::Equal => GameResult::Tie,
            Ordering::Greater => GameResult::Loss,
        },
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
        "{}: {}, {}: {}, Ties: {}",
        p1_priorities.name,
        wins.load(std::sync::atomic::Ordering::Relaxed),
        p2_priorities.name,
        losses.load(std::sync::atomic::Ordering::Relaxed),
        ties.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("Elapsed time: {:?}", elapsed_time);
}

fn main() {
    let p1_name = "chapel_money";
    let p2_name = "big_money_ultimate";
    let action_play = core::strategy::get_priority_list("action_play_priority".to_owned())
        .expect("Error Loading Action Play priority:");
    let treasure_play = core::strategy::get_priority_list("treasure_play_priority".to_owned())
        .expect("Error Loading Treasure Play priority:");
    let trash_priority = core::strategy::get_priority_list("trash_priority".to_owned())
        .expect("Error Loading Treasure Play priority:");
    let p1_buy_priority = core::strategy::get_priority_list(p1_name.to_owned())
        .expect("Error Loading Action Play priority:");
    let chapel_money = core::strategy::get_priority_list(p2_name.to_owned())
        .expect("Error Loading Action Play priority:");
    let p1_priorities = PlayerPriorities {
        name: p1_name,
        buy_priority: &p1_buy_priority,
        action_play_priority: &action_play,
        treasure_play_priority: &treasure_play,
        trash_priority:&trash_priority,
    };
    let p2_priorities = PlayerPriorities {
        name: p2_name,
        buy_priority: &chapel_money,
        action_play_priority: &action_play,
        treasure_play_priority: &treasure_play,
        trash_priority:&trash_priority,
    };

    //run_game(true, true, &p1_priorities, &p2_priorities);

    multi_threaded_tokio(100000, &p1_priorities, &p2_priorities);
}
