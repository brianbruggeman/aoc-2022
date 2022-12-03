use std::fmt;

use super::{moves::*, outcome::Outcome};

pub enum Player {
    Other(Move),
    Me(Move),
    NoOne,
}

impl Player {
    pub fn to_score(&self) -> usize {
        if let Self::Me(my_move) = self {
            my_move.to_score()
        } else {
            0
        }
    }
    pub fn with_outcome<O>(&self, outcome: O) -> Player
    where
        O: Into<Outcome>,
    {
        let outcome = outcome.into();
        match self {
            Self::Other(other_move) => Self::Me(other_move.with_outcome(outcome)),
            _ => unreachable!(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::NoOne
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(player_move) => write!(f, "Other({})", player_move),
            Self::Me(player_move) => write!(f, "Me({})", player_move),
            _ => write!(f, "No One"),
        }
    }
}
