use std::cmp::{Eq, PartialOrd};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use num::traits::Unsigned;

use coord_2d::Coord2D;

pub struct Grid<T: Copy + Display + PartialEq> {
    inner: Vec<Vec<T>>,
    n_rows: usize,
    n_cols: usize,
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

    pub fn get<S: Unsigned + Copy + PartialOrd + Eq + Hash + PartialOrd<usize> + Into<usize>>(
        &self,
        coord: Coord2D<S>,
    ) -> Option<T> {
        if coord.row >= self.n_rows || coord.col >= self.n_cols {
            None
        } else {
            Some(self.inner[coord.row.into()][coord.col.into()])
        }
    }

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

    pub fn print(&self) {
        for line in self.inner.iter() {
            println!("{}", line.iter().map(|t| t.to_string()).collect::<String>());
        }
    }
}

pub fn from_line_iter<T: Copy + Display + PartialEq + From<char>>(
    input: impl Iterator<Item = Result<String, std::io::Error>>,
) -> Grid<T> {
    let result: Vec<Vec<T>> = input
        .into_iter()
        .map(|row| {
            let row = row.unwrap();
            row.chars().map(|c| T::from(c)).collect::<Vec<T>>()
        })
        .collect();

    Grid::new(result)
}
