use std::fmt;

use super::outcome::Outcome;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Undefined,
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn to_score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
            _ => 0,
        }
    }
    pub fn with_outcome<O>(&self, outcome: O) -> Self
    where
        O: Into<Outcome>,
    {
        let outcome = outcome.into();
        match &outcome {
            Outcome::Draw => *self,
            Outcome::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
                _ => Self::Undefined,
            },
            Outcome::Loss => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
                _ => Self::Undefined,
            },
            _ => Self::Undefined,
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<&str> for Move {
    fn from(other: &str) -> Self {
        match other.to_ascii_lowercase().as_str() {
            "x" | "a" => Self::Rock,
            "y" | "b" => Self::Paper,
            "z" | "c" => Self::Scissors,
            _ => Self::Undefined,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rock => write!(f, "{}", "Rock"),
            Self::Paper => write!(f, "{}", "Paper"),
            Self::Scissors => write!(f, "{}", "Scissors"),
            _ => write!(f, "{}", "Undefined"),
        }
    }
}
