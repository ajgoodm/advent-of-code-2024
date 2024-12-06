use std::collections::HashSet;

use coord_2d::Coord2D;
use direction::CardinalDirection;
use grid::{from_line_iter, Grid};
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_6/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid: Grid<char> = from_line_iter(input);
    println!("part 1: {}", part_1_inner(grid));
}

fn part_1_inner(grid: Grid<char>) -> usize {
    let mut map = Map::new(grid);
    let mut positions: HashSet<Coord2D<usize>> = HashSet::new();
    positions.insert(map.position.clone());
    while let Some(next) = map.next() {
        positions.insert(next);
    }

    positions.len()
}

struct Map {
    position: Coord2D<usize>,
    direction: CardinalDirection,
    grid: Grid<char>,
}

impl Map {
    fn next(&mut self) -> Option<Coord2D<usize>> {
        match self.position.adjacent(&self.direction) {
            None => None,
            Some(forward) => match self.grid.get(&forward) {
                Some(c) => match c {
                    '#' => {
                        self.direction = self.direction.turn_right();
                        Some(self.position.clone())
                    }
                    '.' => {
                        self.position = forward.clone();
                        Some(forward)
                    }
                    _ => panic!("I think I saw a 2! {}", c),
                },
                None => None,
            },
        }
    }

    fn new(mut grid: Grid<char>) -> Self {
        let mut position: Option<Coord2D<usize>> = None;
        let mut direction: Option<CardinalDirection> = None;

        for (row_idx, row) in grid.rows().enumerate() {
            for (col_idx, c) in row.into_iter().enumerate() {
                match c {
                    '<' | '^' | '>' | 'v' => {
                        position = Some(Coord2D::new(row_idx, col_idx));
                        direction = Some(CardinalDirection::from_char(c));
                    }
                    _ => (),
                }
            }
        }

        match (position, direction) {
            (Some(p), Some(d)) => {
                grid.set('.', p.row, p.col);
                Self {
                    position: p,
                    direction: d,
                    grid,
                }
            }
            _ => panic!("We didn't find the guard!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}
}
