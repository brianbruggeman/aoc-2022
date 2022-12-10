use std::{collections::HashSet, fmt, ops};

use tracing::{debug, trace};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    y: i32,
    x: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(y={}, x={})", self.y, self.x)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { y: 0, x: 0 }
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

pub fn count_tail_position(moves: &str) -> usize {
    let mut head_position = Position::default();
    let mut tail_position = Position::default();
    let mut tail_positions = vec![head_position];

    parse_moves(moves).iter().for_each(|head_move| {
        let old_position = head_position;
        head_position = calculate_position(&head_position, head_move);
        trace!("Head: {old_position} + {head_move} => {head_position}");
        let tail_direction = calculate_direction(&head_position, &tail_position);
        if !is_touching(&head_position, &tail_position) {
            let old_tail = tail_position;
            tail_position = tail_position + Update::new(tail_direction, 1);
            tail_positions.push(tail_position);
            trace!("{old_tail} + {tail_direction} => {tail_position}");
        } else {
            trace!("{tail_position} + {tail_direction} => No Move");
        }
    });
    let steps = tail_positions
        .iter()
        .map(|p| *p)
        .collect::<HashSet<Position>>();
    steps.iter().count()
}

pub fn calculate_position(position: &Position, update: &Update) -> Position {
    *position + *update
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

#[derive(Copy, Clone, Debug)]
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
            (0..size).map(move |_| Update::new(direction.clone(), 1))
        })
        .inspect(|u| trace!("Move: {u}"))
        .collect::<Vec<_>>()
}
