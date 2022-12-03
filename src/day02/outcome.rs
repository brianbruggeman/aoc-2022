pub enum Outcome {
    Loss,
    Win,
    Draw,
    Unplayed,
}

impl Default for Outcome {
    fn default() -> Self {
        Outcome::Unplayed
    }
}

impl From<&str> for Outcome {
    fn from(other: &str) -> Self {
        match other.to_ascii_lowercase().as_str() {
            "x" => Outcome::Loss,
            "y" => Outcome::Draw,
            "z" => Outcome::Win,
            _ => Outcome::Unplayed,
        }
    }
}
