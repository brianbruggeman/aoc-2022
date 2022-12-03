pub mod moves;
pub mod outcome;
pub mod player;
pub mod rock_paper;

pub fn run() {
    let moves = include_str!("input1.txt");
    println!("Day 2:");
    println!("    Guessed: score {}", rock_paper::play_round_1(moves));
    println!("    Actual: score {}", rock_paper::play_round_2(moves));
}
