use std::cmp::{Eq, PartialOrd};
use std::hash::Hash;

use num::traits::Unsigned;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Coord2D<T: Unsigned + PartialOrd + Eq + Hash + Copy> {
    pub row: T,
    pub col: T,
}

impl<T: Unsigned + PartialOrd + Eq + Copy + Hash> Coord2D<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }

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
