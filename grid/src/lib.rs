use std::cmp::{Eq, PartialOrd};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use num::Integer;

use coord_2d::Coord2D;

#[derive(Clone, Debug)]
pub struct Grid<T: Copy + Display + PartialEq> {
    inner: Vec<Vec<T>>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl<T: Copy + Display + PartialEq> Grid<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Self {
        let n_rows = inner.len();
        let n_cols = inner[0].len();

        Self {
            inner,
            n_rows,
            n_cols,
        }
    }

    pub fn set(&mut self, val: T, row_idx: usize, col_idx: usize) {
        if row_idx >= self.n_rows || col_idx >= self.n_cols {
            panic!(
                "Invalid set coord ({}, {}); n_rows: {}, n_cols: {}",
                row_idx, col_idx, self.n_rows, self.n_cols
            )
        }

        self.inner[row_idx][col_idx] = val;
    }

    pub fn get<S>(&self, coord: &Coord2D<S>) -> Option<T>
    where
        S: Integer + Copy + PartialOrd + Eq + Hash + TryInto<usize>,
        <S as TryInto<usize>>::Error: std::fmt::Debug,
    {
        let row: usize = coord.row.try_into().unwrap();
        let col: usize = coord.col.try_into().unwrap();

        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(self.inner[row][col])
        }
    }

    /// Find every (row, col) whose value matches needle
    pub fn find(&self, needle: T) -> HashSet<Coord2D<usize>> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &val)| val == needle)
                    .map(move |(col_idx, _)| Coord2D::new(row_idx, col_idx))
            })
            .collect::<HashSet<Coord2D<usize>>>()
    }

    /// Find every (row, col) whose value matches needle
    pub fn find_one(&self, needle: T) -> Coord2D<usize> {
        let all = self
            .inner
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &val)| val == needle)
                    .map(move |(col_idx, _)| Coord2D::new(row_idx, col_idx))
            })
            .collect::<Vec<Coord2D<usize>>>();

        if all.len() != 1 {
            panic!("too many or too few to find just one");
        }

        all.into_iter().next().unwrap()
    }

    pub fn row(&self, row_idx: usize) -> Vec<T> {
        if row_idx >= self.n_rows {
            panic!(
                "That ({}), is not a real row (max={})",
                row_idx, self.n_rows
            );
        }
        self.inner[row_idx].to_vec()
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
        (0..self.n_rows).map(|row_idx| self.row(row_idx))
    }

    pub fn col(&self, col_idx: usize) -> Vec<T> {
        if col_idx >= self.n_cols {
            panic!(
                "That ({}), is not a real column (max={})",
                col_idx, self.n_cols
            );
        }

        (0..self.n_rows)
            .map(|row_idx| self.inner[row_idx][col_idx])
            .collect()
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
        (0..self.n_cols).map(|col_idx| self.col(col_idx))
    }

    pub fn se_diagonal(&self, row_idx: usize, col_idx: usize) -> Vec<T> {
        if row_idx != 0 && col_idx != 0 {
            panic!(
                "SE diagonals must start from the left most column or top row, {}, {}",
                row_idx, col_idx
            );
        }

        if row_idx >= self.n_rows || col_idx >= self.n_cols {
            panic!("bad row or col idx");
        }

        (row_idx..self.n_rows)
            .zip_longest(col_idx..self.n_cols)
            .filter_map(|x| match x {
                Both(row, col) => Some(self.get(&Coord2D::new(row, col)).unwrap()),
                _ => None,
            })
            .collect()
    }

    pub fn se_diagonals(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
        let mut row_cols: Vec<(usize, usize)> =
            (0..self.n_rows).rev().map(|row_idx| (row_idx, 0)).collect();
        row_cols.extend((1..self.n_cols).map(|col_idx| (0, col_idx)));
        row_cols
            .into_iter()
            .map(|(row_idx, col_idx)| self.se_diagonal(row_idx, col_idx))
    }

    pub fn ne_diagonal(&self, row_idx: usize, col_idx: usize) -> Vec<T> {
        if row_idx != self.n_rows - 1 && col_idx != 0 {
            panic!("NE diagonals must start from the left most column or bottom row");
        }

        if row_idx >= self.n_rows || col_idx >= self.n_cols {
            panic!("bad row or col idx");
        }

        (0..=row_idx)
            .rev()
            .zip_longest(col_idx..self.n_cols)
            .filter_map(|x| match x {
                Both(row, col) => Some(self.get(&Coord2D::new(row, col)).unwrap()),
                _ => None,
            })
            .collect()
    }

    pub fn ne_diagonals(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T> {
        let mut row_cols: Vec<(usize, usize)> =
            (0..self.n_rows).map(|row_idx| (row_idx, 0)).collect();
        row_cols.extend((1..self.n_cols).map(|col_idx| (self.n_rows - 1, col_idx)));
        row_cols
            .into_iter()
            .map(|(row_idx, col_idx)| self.ne_diagonal(row_idx, col_idx))
    }

    pub fn coords_and_vals<S>(&self) -> impl Iterator<Item = (Coord2D<S>, T)> + use<'_, T, S>
    where
        S: Integer + PartialOrd + Eq + Hash + Copy + TryFrom<usize>,
        <S as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        (0..self.n_rows)
            .cartesian_product(0..self.n_cols)
            .map(|(row_idx, col_idx)| {
                (
                    Coord2D::new(row_idx.try_into().unwrap(), col_idx.try_into().unwrap()),
                    self.inner[row_idx][col_idx],
                )
            })
    }

    pub fn print(&self) {
        for line in self.inner.iter() {
            println!("{}", line.iter().map(|t| t.to_string()).collect::<String>());
        }
    }
}

impl Grid<char> {
    pub fn from_line_iter(input: impl Iterator<Item = String>) -> Self {
        let result: Vec<Vec<char>> = input
            .into_iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect();

        Self::new(result)
    }

    pub fn into_numeric_type<S: Copy + Display + PartialEq + TryFrom<u32>>(self) -> Grid<S>
    where
        <S as TryFrom<u32>>::Error: std::fmt::Debug,
    {
        let inner: Vec<Vec<S>> = self
            .inner
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| S::try_from(c.to_digit(10u32).unwrap()).unwrap())
                    .collect::<Vec<S>>()
            })
            .collect();

        Grid {
            inner,
            n_rows: self.n_rows,
            n_cols: self.n_cols,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let grid = Grid::from_line_iter(["abc", "abc", "abc"].into_iter().map(|x| x.to_string()));
        assert_eq!(
            grid.find('a'),
            HashSet::from_iter([Coord2D::new(0, 0), Coord2D::new(1, 0), Coord2D::new(2, 0)])
        );
    }

    #[test]
    fn test_rows_etc() {
        let grid = Grid::from_line_iter(["abc", "def", "ghi"].into_iter().map(|x| x.to_string()));

        assert_eq!(
            grid.rows()
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            ["abc", "def", "ghi"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            grid.cols()
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            ["adg", "beh", "cfi"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            grid.se_diagonals()
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            ["g", "dh", "aei", "bf", "c"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            grid.ne_diagonals()
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            ["a", "db", "gec", "hf", "i"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );
    }
}
