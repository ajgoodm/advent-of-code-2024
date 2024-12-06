#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn turn_right(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::North,
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("bad direction char {}", c),
        }
    }
}
