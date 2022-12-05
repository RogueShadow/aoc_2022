use std::time::Instant;
use RPS::*;
use WLD::*;

#[derive(Debug,Copy,Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
    Unknown,
}
#[derive(Debug,Copy,Clone)]
pub enum WLD {
    Win,
    Lose,
    Draw,
    Unknown,
}
pub fn day2(data: String) {
    let timer = Instant::now();
    // part 1.
    let input = data.split('\n')
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
        .map(|(them,you)| (map_data_rps(them),map_data_rps(you)))
        .collect::<Vec<_>>();
    let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
    println!("Completed in {:?}",timer.elapsed());
    println!("Score: {:?}",score);

    // part 2.
    let timer = Instant::now();
    let input = data.split('\n')
        .map(|s| s.split(" ").collect::<Vec<_>>())
        .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
        .map(|(them,you)| (map_data_rps(them),map_win_lose_draw(you)))
        .map(|(them, you)| (them, play_to_wld(&them,&you)))
        .collect::<Vec<_>>();
    let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
    println!("Completed in {:?}",timer.elapsed());
    println!("Score: {:?}",score);
}
pub fn map_win_lose_draw(value: &str) -> WLD {
    match value {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => WLD::Unknown,
    }
}
pub fn map_data_rps(value: &str) -> RPS {
    match value {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => RPS::Unknown
    }
}
pub fn play_to_wld(them: &RPS, you: &WLD) -> RPS {
    match (them, you) {
        (Rock, Lose) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Lose) => Paper,
        (Rock, Draw) => Rock,
        (Paper, Draw) => Paper,
        (Scissors, Draw) => Scissors,
        (Rock, Win) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
        _ => RPS::Unknown
    }
}
pub fn score_round(them: &RPS, you: &RPS) -> i32 {
    match (them,you) {
        (Rock,Rock) => 1 + 3,
        (Paper,Rock) => 1 + 0,
        (Scissors,Rock) => 1 + 6,
        (Rock,Paper) => 2 + 6,
        (Paper,Paper) => 2 + 3,
        (Scissors,Paper) => 2 + 0,
        (Rock,Scissors) => 3 + 0,
        (Paper,Scissors) => 3 + 6,
        (Scissors,Scissors) => 3 + 3,
        _ => 0,
    }
}
