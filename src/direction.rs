// 2048 - Terminal, 15/01/2023
// Direction: Contains the enum of possible directions and invalid for invalid input
// (c) aichingert

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid
}

impl Direction {
    pub fn from_string(line: &str) -> Self {
        match line.to_lowercase().as_str() {
            "up" | "u" => Direction::Up,
            "down" | "d" => Direction::Down,
            "left" | "l" => Direction::Left,
            "right" | "r" => Direction::Right,
            _ => Direction::Invalid,
        }
    }
}
