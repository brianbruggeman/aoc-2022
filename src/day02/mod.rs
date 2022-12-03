pub mod moves;
pub mod outcome;
pub mod player;
pub mod rock_paper;

pub fn run() {
    let moves = include_str!("input1.txt");
    println!("Day 2:");
    println!("    Part 1: score {}", rock_paper::play_round_1(moves));
    println!("    Part 2: score {}", rock_paper::play_round_2(moves));
}
