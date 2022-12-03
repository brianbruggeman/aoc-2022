use super::moves::*;
use super::player::*;

const LOST_POINTS: usize = 0;
const DRAW_POINTS: usize = 3;
const WON_POINTS: usize = 6;

pub fn play_round_1(moves: &str) -> usize {
    moves
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| extract_moves(line))
        .map(|(other, me)| score_move(other, me))
        .sum()
}

pub fn play_round_2(moves: &str) -> usize {
    moves
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| extract_new_rules_moves(line))
        .map(|(other, me)| score_move(other, me))
        .sum()
}

fn score_move(other: Player, me: Player) -> usize {
    let move_score = me.to_score();
    let play_score = match (&other, &me) {
        (Player::Other(Move::Paper), Player::Me(Move::Rock)) => LOST_POINTS,
        (Player::Other(Move::Paper), Player::Me(Move::Scissors)) => WON_POINTS,
        (Player::Other(Move::Rock), Player::Me(Move::Scissors)) => LOST_POINTS,
        (Player::Other(Move::Rock), Player::Me(Move::Paper)) => WON_POINTS,
        (Player::Other(Move::Scissors), Player::Me(Move::Paper)) => LOST_POINTS,
        (Player::Other(Move::Scissors), Player::Me(Move::Rock)) => WON_POINTS,
        (Player::Other(other_move), Player::Me(my_move)) if other_move == my_move => DRAW_POINTS,
        _ => unreachable!("{} vs {}", other, me),
    };
    move_score + play_score
}

fn extract_moves(line: &str) -> (Player, Player) {
    let mut other = Player::NoOne;
    let mut me = Player::NoOne;
    for (index, player_move) in line.trim().split(' ').into_iter().enumerate() {
        let player_move = Move::from(player_move);
        match index {
            0 => other = Player::Other(player_move),
            _ => me = Player::Me(player_move),
        }
    }
    (other, me)
}

fn extract_new_rules_moves(line: &str) -> (Player, Player) {
    let mut other = Move::Undefined;
    let mut me = Move::Undefined;
    for (index, symbol) in line.trim().split(' ').into_iter().enumerate() {
        match index {
            0 => other = Move::from(symbol),
            _ => me = other.with_outcome(symbol),
        }
    }
    (Player::Other(other), Player::Me(me))
}
