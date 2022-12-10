use std::collections::BTreeMap;
use std::fmt;

use tracing::{debug, trace};

pub fn find_highest_scenic_score(tree: &str) -> usize {
    let tree = parse_tree(tree);
    tree.iter()
        .filter(|(_pos, cell)| matches!(cell, Cell::Value(_)))
        .map(|(pos, _cell)| calculate_scenic_score(pos, &tree))
        .max()
        .unwrap()
}

pub fn calculate_scenic_score(position: &Position, tree: &BTreeMap<Position, Cell>) -> usize {
    let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    let cell = tree.get(position).unwrap();
    let score = directions
        .iter()
        .map(|direction| {
            let mut count = 0;
            for ray_position in cast_ray(position, *direction, tree)
                .iter()
                .filter(|p| p != &position)
            {
                count += 1;
                let ray_cell = tree.get(ray_position).unwrap();
                trace!(" --- {position}:{cell}:{direction:?} => {ray_cell}");
                if ray_cell.value() >= cell.value() {
                    break;
                }
            }
            trace!("{position}:{cell}:{direction:?} => {count}");
            count
        })
        .product();
    trace!("{position}:{cell} => {score}");
    score
}

pub fn find_trees_visible(tree: &str) -> usize {
    let tree = parse_tree(tree);
    let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
    let visible_edges = tree
        .iter()
        .filter(|(_p, c)| matches!(c, Cell::Edge(_)))
        .count();
    let visible_internal = tree
        .iter()
        .filter(|(_p, c)| matches!(c, Cell::Value(_)))
        .filter(|(pos, cell)| {
            let visible = directions.iter().any(|d| {
                let all_okay = cast_ray(pos, *d, &tree)
                    .iter()
                    .filter(|p| p != pos)
                    .all(|p| {
                        let ray_cell = tree.get(p).unwrap();
                        let cmp = cell.value() > ray_cell.value();
                        trace!("[{pos}, {cell}, {d:?}] L:{cell} > C:{p}:{ray_cell} ? {cmp}");
                        cmp
                    });
                all_okay
            });
            debug!("{pos} visible? {visible}");
            visible
        })
        .count();
    visible_internal + visible_edges
}

pub fn cast_ray(start: &Position, direction: Direction, tree: &BTreeMap<Position, Cell>) -> Vec<Position> {
    let mut ray = vec![*start];
    let mut next = *start;
    loop {
        trace!("Start: {start} | Next: {next} | ray = {ray:?}");
        match get_next_in_ray(&next, direction, tree) {
            Some(p) => {
                next = p;
                ray.push(p)
            }
            None => break,
        }
    }
    ray
}

pub fn get_next_in_ray(start: &Position, direction: Direction, tree: &BTreeMap<Position, Cell>) -> Option<Position> {
    let next_pos = match direction {
        Direction::North => Position::new(start.y - 1, start.x),
        Direction::South => Position::new(start.y + 1, start.x),
        Direction::East => Position::new(start.y, start.x + 1),
        Direction::West => Position::new(start.y, start.x - 1),
    };
    match tree.contains_key(&next_pos) {
        true => Some(next_pos),
        false => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cell {
    Value(u8),
    Edge(u8),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "V({v})"),
            Self::Edge(v) => write!(f, "E({v})"),
        }
    }
}

impl Cell {
    pub fn value(&self) -> usize {
        match self {
            Self::Value(v) => *v as usize,
            Self::Edge(v) => *v as usize,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub y: i16,
    pub x: i16,
}

impl Position {
    pub fn new<X: Into<i16>, Y: Into<i16>>(y: Y, x: X) -> Self {
        Self { x: x.into(), y: y.into() }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(y={}, x={})", self.y, self.x)
    }
}

impl<X: Into<i16>, Y: Into<i16>> From<(Y, X)> for Position {
    fn from((y, x): (Y, X)) -> Self {
        let x = x.into();
        let y = y.into();
        Self { x, y }
    }
}

pub fn parse_tree(tree: &str) -> BTreeMap<Position, Cell> {
    let mut map = BTreeMap::new();
    let line_count = tree
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .count() as i16;
    for (y, line) in tree.split('\n').enumerate() {
        let y = y as i16;
        let col_count = line.chars().count() as i16;
        for (x, c) in line.chars().enumerate() {
            let x = x as i16;
            let pos = Position::new(y, x);
            let v = c
                .to_string()
                .parse::<u8>()
                .expect("Could not parse value");
            let cell = match (x, y) {
                (x, y) if x == 0 || y == 0 || x == col_count - 1 || y == line_count - 1 => Cell::Edge(v),
                _ => Cell::Value(v),
            };
            map.insert(pos, cell);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32", &[((0, 0), Cell::Edge(3)), ((0, 1), Cell::Edge(2))])]
    #[case("321\n456\n789", &[
        ((0, 0), Cell::Edge(3)), ((0, 1), Cell::Edge(2)), ((0, 2), Cell::Edge(1)),
        ((1, 0), Cell::Edge(4)), ((1, 1), Cell::Value(5)), ((1, 2), Cell::Edge(6)),
        ((2, 0), Cell::Edge(7)), ((2, 1), Cell::Edge(8)), ((2, 2), Cell::Edge(9))])]
    pub fn test_parse_tree(#[case] tree: &str, #[case] expected: &[((i16, i16), Cell)]) {
        let expected = expected
            .iter()
            .map(|(p, c)| {
                let p: Position = p.clone().into();
                let c: Cell = c.clone().into();
                (p, c)
            })
            .collect::<BTreeMap<Position, Cell>>();
        let actual = parse_tree(tree)
            .iter()
            .map(|(p, c)| (*p, *c))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("3210", (0, 0), Direction::East, &[(0, 0), (0, 1), (0, 2), (0, 3)])]
    pub fn test_cast_array(#[case] tree: &str, #[case] start: (i16, i16), #[case] direction: Direction, #[case] expected: &[(i16, i16)]) {
        let tree = parse_tree(tree);
        let expected = expected
            .iter()
            .map(|(y, x)| Position::new(*y, *x))
            .collect::<Vec<Position>>();
        let start = Position::from(start);
        let actual = cast_ray(&start, direction, &tree);
        assert_eq!(actual, expected)
    }
}
