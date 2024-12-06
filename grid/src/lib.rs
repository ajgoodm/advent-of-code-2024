use std::cmp::{Eq, PartialOrd};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use num::traits::Unsigned;

use coord_2d::Coord2D;

#[derive(Clone)]
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

    pub fn get<S: Unsigned + Copy + PartialOrd + Eq + Hash + PartialOrd<usize> + Into<usize>>(
        &self,
        coord: &Coord2D<S>,
    ) -> Option<T> {
        if coord.row >= self.n_rows || coord.col >= self.n_cols {
            None
        } else {
            Some(self.inner[coord.row.into()][coord.col.into()])
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

    pub fn print(&self) {
        for line in self.inner.iter() {
            println!("{}", line.iter().map(|t| t.to_string()).collect::<String>());
        }
    }
}

pub fn from_line_iter<T: Copy + Display + PartialEq + From<char>>(
    input: impl Iterator<Item = String>,
) -> Grid<T> {
    let result: Vec<Vec<T>> = input
        .into_iter()
        .map(|row| row.chars().map(|c| T::from(c)).collect::<Vec<T>>())
        .collect();

    Grid::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows_etc() {
        let grid: Grid<char> =
            from_line_iter(["abc", "def", "ghi"].into_iter().map(|x| x.to_string()));

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
