use std::cmp::{Eq, PartialOrd};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops;
use std::str::FromStr;

use num::traits::Unsigned;
use num::{Integer, Signed};

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

    pub fn mul_scalar(&self, v: T) -> Self {
        Self::new(self.row * v, self.col * v)
    }

    pub fn is_nonnegative(&self) -> bool {
        self.row >= T::zero() && self.col >= T::zero()
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
        let n_streets = {
            if self.row > other.row {
                self.row - other.row
            } else {
                other.row - self.row
            }
        };

        let n_avenues = {
            if self.col > other.col {
                self.col - other.col
            } else {
                other.col - self.col
            }
        };

        n_streets + n_avenues
    }
}

impl<T: Integer + PartialOrd + Eq + Copy + Hash> ops::Add<Coord2D<T>> for Coord2D<T> {
    type Output = Self;

    fn add(self, _rhs: Coord2D<T>) -> Self {
        Coord2D::new(self.row + _rhs.row, self.col + _rhs.col)
    }
}

impl<T: Integer + Signed + PartialOrd + Eq + Copy + Hash> ops::Sub<Coord2D<T>> for Coord2D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coord2D::new(self.row - other.row, self.col - other.col)
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

    pub fn north_east(&self) -> Option<Self> {
        if self.row > T::zero() {
            Some(Self::new(self.row - T::one(), self.col + T::one()))
        } else {
            None
        }
    }

    pub fn east(&self) -> Self {
        Self::new(self.row, self.col + T::one())
    }

    pub fn south_east(&self) -> Self {
        Self::new(self.row + T::one(), self.col + T::one())
    }

    pub fn south(&self) -> Self {
        Self::new(self.row + T::one(), self.col)
    }

    pub fn south_west(&self) -> Option<Self> {
        if self.col > T::zero() {
            Some(Self::new(self.row + T::one(), self.col - T::one()))
        } else {
            None
        }
    }

    pub fn west(&self) -> Option<Self> {
        if self.col > T::zero() {
            Some(Self::new(self.row, self.col - T::one()))
        } else {
            None
        }
    }

    pub fn north_west(&self) -> Option<Self> {
        if self.row > T::zero() && self.col > T::zero() {
            Some(Self::new(self.row - T::one(), self.col - T::one()))
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCoord2DUsizeError;

impl FromStr for Coord2D<usize> {
    type Err = ParseCoord2DUsizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");
        Ok(Coord2D::new(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ))
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
