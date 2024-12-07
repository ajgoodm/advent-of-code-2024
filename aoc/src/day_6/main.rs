use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

use coord_2d::Coord2D;
use direction::CardinalDirection;
use grid::{from_line_iter, Grid};
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_6/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_6/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid: Grid<char> = from_line_iter(input);
    println!("part 1: {}", part_1_inner(grid));
}

fn part_2(input: AocBufReader) {
    let grid: Grid<char> = from_line_iter(input);
    println!("part 2: {}", part_2_inner(grid));
}

fn part_1_inner(grid: Grid<char>) -> usize {
    let map = Map::new(grid);
    match run_to_completion(map) {
        CompletionCondition::WalkedOffTheMap(n_sites) => n_sites,
        CompletionCondition::Looped => panic!("we weren't supposed to loop!"),
    }
}

fn part_2_inner(grid: Grid<char>) -> usize {
    let map = Map::new(grid);

    let row_cols: Vec<(usize, usize)> = (0..map.grid.n_rows)
        .cartesian_product(0..map.grid.n_cols)
        .filter(|(r, c)| map.position != Coord2D::new(*r, *c))
        .collect();

    row_cols
        .par_iter()
        .map(|(row_idx, col_idx)| {
            let mut new_map = map.clone();
            new_map.grid.set('#', *row_idx, *col_idx);
            run_to_completion(new_map)
        })
        .filter(|x| match x {
            CompletionCondition::Looped => true,
            CompletionCondition::WalkedOffTheMap(_) => false,
        })
        .count()
}

enum CompletionCondition {
    WalkedOffTheMap(usize),
    Looped,
}

fn run_to_completion(mut map: Map) -> CompletionCondition {
    let mut positions: HashSet<Coord2D<usize>> = HashSet::new();
    let mut state: HashSet<(Coord2D<usize>, CardinalDirection)> = HashSet::new();
    positions.insert(map.position.clone());
    while let Some(next) = map.next() {
        if state.contains(&(next.clone(), map.direction)) {
            return CompletionCondition::Looped;
        } else {
            state.insert((map.position.clone(), map.direction));
            positions.insert(next);
        }
    }

    CompletionCondition::WalkedOffTheMap(positions.len())
}

#[derive(Clone)]
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
    fn test_part1_2() {
        let grid: Grid<char> = from_line_iter(
            [
                "....#.....",
                ".........#",
                "..........",
                "..#.......",
                ".......#..",
                "..........",
                ".#..^.....",
                "........#.",
                "#.........",
                "......#...",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(part_2_inner(grid), 6);
    }
}
