use std::cmp::{Eq, PartialOrd};
use std::fmt::Debug;
use std::hash::Hash;

use num::traits::Unsigned;
use num::Integer;

use direction::CardinalDirection;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Coord2D<T: Integer + PartialOrd + Eq + Hash + Copy> {
    pub row: T,
    pub col: T,
}

impl<T: Integer + PartialOrd + Eq + Copy + Hash> Coord2D<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }

    pub fn from_signed<S>(row: S, col: S) -> Self
    where
        S: TryInto<T>,
        <S as TryInto<T>>::Error: std::fmt::Debug,
    {
        Coord2D::new(row.try_into().unwrap(), col.try_into().unwrap())
    }
}

impl<T: Integer + Unsigned + PartialOrd + Eq + Copy + Hash> Coord2D<T> {
    pub fn neighbors(&self) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        if self.row > T::zero() {
            result.extend([
                Self::new(self.row - T::one(), self.col),
                Self::new(self.row - T::one(), self.col + T::one()),
            ]);
            if self.col > T::zero() {
                result.push(Self::new(self.row - T::one(), self.col - T::one()));
            }
        }

        if self.col > T::zero() {
            result.extend([
                Self::new(self.row, self.col - T::one()),
                Self::new(self.row + T::one(), self.col - T::one()),
            ]);
        }

        result.extend([
            Self::new(self.row + T::one(), self.col),
            Self::new(self.row, self.col + T::one()),
            Self::new(self.row + T::one(), self.col + T::one()),
        ]);

        result
    }

    pub fn cardinal_neighbors(&self) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        if self.row > T::zero() {
            result.push(Self::new(self.row - T::one(), self.col));
        }

        if self.col > T::zero() {
            result.push(Self::new(self.row, self.col - T::one()));
        }

        result.extend([
            Self::new(self.row + T::one(), self.col),
            Self::new(self.row, self.col + T::one()),
        ]);

        result
    }

    pub fn north(&self) -> Option<Self> {
        if self.row > T::zero() {
            Some(Self::new(self.row - T::one(), self.col))
        } else {
            None
        }
    }

    pub fn east(&self) -> Self {
        Self::new(self.row, self.col + T::one())
    }

    pub fn south(&self) -> Self {
        Self::new(self.row + T::one(), self.col)
    }

    pub fn west(&self) -> Option<Self> {
        if self.col > T::zero() {
            Some(Self::new(self.row, self.col - T::one()))
        } else {
            None
        }
    }

    pub fn adjacent(&self, direction: &CardinalDirection) -> Option<Self> {
        match direction {
            CardinalDirection::North => self.north(),
            CardinalDirection::East => Some(self.east()),
            CardinalDirection::South => Some(self.south()),
            CardinalDirection::West => self.west(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_neighbors() {
        let x: Coord2D<usize> = Coord2D::new(1, 1);
        let neighbors: HashSet<Coord2D<usize>> = HashSet::from_iter(x.neighbors());
        assert_eq!(neighbors.len(), 8);

        let x: Coord2D<usize> = Coord2D::new(0, 0);
        let neighbors: HashSet<Coord2D<usize>> = HashSet::from_iter(x.neighbors());
        assert_eq!(neighbors.len(), 3);

        let x: Coord2D<usize> = Coord2D::new(0, 1);
        let neighbors: HashSet<Coord2D<usize>> = HashSet::from_iter(x.neighbors());
        assert_eq!(neighbors.len(), 5);
    }
}
