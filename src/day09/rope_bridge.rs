use std::{collections::HashSet, fmt, ops};

use tracing::{debug, trace};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    y: i32,
    x: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(y={}, x={})", self.y, self.x)
    }
}

impl Position {
    pub fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }
}

impl ops::Add<Update> for Position {
    type Output = Position;

    fn add(self, rhs: Update) -> Self::Output {
        let direction = rhs.direction;
        let distance = rhs.distance as i32;
        match direction {
            Direction::Stationary => self,
            Direction::Right => Position::new(self.y, self.x + distance),
            Direction::Left => Position::new(self.y, self.x - distance),
            Direction::Up => Position::new(self.y - distance, self.x),
            Direction::Down => Position::new(self.y + distance, self.x),
            Direction::DownRight => Position::new(self.y + distance, self.x + distance),
            Direction::DownLeft => Position::new(self.y + distance, self.x - distance),
            Direction::UpLeft => Position::new(self.y - distance, self.x - distance),
            Direction::UpRight => Position::new(self.y - distance, self.x + distance),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Update {
    direction: Direction,
    distance: usize,
}

impl Update {
    fn new(direction: Direction, distance: usize) -> Self {
        Self { direction, distance }
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

pub fn count_tail_position(moves: &str, knot_count: usize) -> usize {
    let mut positions: Vec<Position> = vec![Position::default(); knot_count];
    let mut tail_positions = vec![Position::default()];

    parse_moves(moves)
        .iter()
        .enumerate()
        .for_each(|(move_id, head_move)| {
            let mut next_update = *head_move;
            let mut last_position = positions[0];
            // Propegate move through chain of knots and record the tail position
            for (position_id, position) in positions.iter_mut().enumerate() {
                debug!("{move_id}:{position_id}: Update: {position} (current)");
                if position_id > 0 {
                    if !is_touching(&last_position, position) {
                        let direction = calculate_direction(&last_position, position);
                        if direction == Direction::Stationary {
                            debug!("{move_id}:{position_id}: No update. Halting updates");
                            break;
                        }
                        next_update = Update::new(direction, 1);
                    } else {
                        debug!("{move_id}:{position_id}: No update. Halting updates");
                        break;
                    }
                }
                if next_update.direction == Direction::Stationary {
                    debug!("{move_id}:{position_id}: No update.");
                    break;
                }
                *position = *position + next_update;
                debug!("{move_id}:{position_id}: Update: {position} **updated**");
                last_position = *position;
                if position_id == knot_count - 1 {
                    debug!("Updated tail: {}", position);
                    tail_positions.push(*position)
                }
            }
            debug!("Positions: {:?}", positions);
        });
    let steps = tail_positions
        .iter()
        .copied()
        .collect::<HashSet<Position>>();
    steps.len()
}

pub fn calculate_direction(head_pos: &Position, tail_pos: &Position) -> Direction {
    if !is_touching(head_pos, tail_pos) {
        match (head_pos, tail_pos) {
            (h, t) if h.x == t.x && h.y == t.y => Direction::Stationary,
            // One of: column or row are different
            (h, t) if h.x > t.x && h.y == t.y => Direction::Right,
            (h, t) if h.x < t.x && h.y == t.y => Direction::Left,
            (h, t) if h.x == t.x && h.y > t.y => Direction::Down,
            (h, t) if h.x == t.x && h.y < t.y => Direction::Up,
            // Both: column and row are different
            (h, t) if h.x > t.x && h.y > t.y => Direction::DownRight,
            (h, t) if h.x > t.x && h.y < t.y => Direction::UpRight,
            (h, t) if h.x < t.x && h.y < t.y => Direction::UpLeft,
            (h, t) if h.x < t.x && h.y > t.y => Direction::DownLeft,
            (_, _) => unreachable!(),
        }
    } else {
        Direction::Stationary
    }
}

pub fn is_touching(pos1: &Position, pos2: &Position) -> bool {
    (pos1.x - 1..=pos1.x + 1).contains(&pos2.x) && (pos1.y - 1..=pos1.y + 1).contains(&pos2.y)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Stationary,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Stationary
    }
}

impl From<&str> for Direction {
    fn from(other: &str) -> Self {
        match other.to_ascii_lowercase().as_str() {
            "r" => Self::Right,
            "u" => Self::Up,
            "d" => Self::Down,
            "l" => Self::Left,
            _ => Self::Stationary,
        }
    }
}

pub fn parse_moves(positions: &str) -> Vec<Update> {
    positions
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| {
            let (direction, size) = line.split_once(' ').unwrap();
            let direction = Direction::from(direction);
            let size = size.parse::<usize>().unwrap();
            (0..size).map(move |_| Update::new(direction, 1))
        })
        .inspect(|u| trace!("Move: {u}"))
        .collect::<Vec<_>>()
}
