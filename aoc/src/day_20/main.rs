use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

use coord_2d::Coord2D;
use grid::Grid;
use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_20/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let map = Map::from_input(input);
    println!("part 1: {}", part_1_inner(map));
}

/// We can consider every barrier position and determine the minimum path
/// length from each side of the barrier. If that length is > 100, then
/// we can consider that barrier position a candidate to check later.
fn part_1_inner(map: Map) -> usize {
    let original_shortest_length = map.shortest_path_length();
    let barrier_positions: Vec<Coord2D<usize>> = map
        .grid
        .find('#')
        .into_iter()
        .filter(|x| {
            x.row > 0 && x.row < map.grid.n_rows - 1 && x.col > 0 && x.col < map.grid.n_cols - 1
        })
        .collect();

    let candidates = filter_candidates(&map, barrier_positions);
    println!("evaluating {} candidates", candidates.len());
    candidates
        .par_iter()
        .filter(|candidate| {
            let mut new_map = map.clone();
            new_map.grid.set('.', candidate.row, candidate.col);
            let new_shortest_length = new_map.shortest_path_length();
            original_shortest_length - new_shortest_length >= 100
        })
        .count()
}

fn filter_candidates(map: &Map, candidates: Vec<Coord2D<usize>>) -> Vec<Coord2D<usize>> {
    println!("starting with {} candidates", candidates.len());
    candidates
        .par_iter()
        .cloned()
        .filter(|candidate| {
            let open_neighbors: Vec<_> = candidate
                .cardinal_neighbors()
                .into_iter()
                .filter(|x| map.grid.get(x) == Some('.'))
                .collect();

            if open_neighbors.len() < 2 {
                false
            } else {
                open_neighbors
                    .into_iter()
                    .combinations(2)
                    .any(|mut start_end| {
                        let end = start_end.pop().unwrap();
                        let start = start_end.pop().unwrap();
                        match shortest_path_length(map, start, HashSet::from([end])) {
                            Some(length) => length >= 100,
                            None => false,
                        }
                    })
            }
        })
        .collect()
}

#[derive(Clone)]
struct Map {
    grid: Grid<char>,
    start: Coord2D<usize>,
    end: Coord2D<usize>,
}

impl Map {
    fn shortest_path_length(&self) -> usize {
        shortest_path_length(self, self.start.clone(), HashSet::from([self.end.clone()])).unwrap()
    }

    fn from_input(input: AocBufReader) -> Self {
        let mut grid = Grid::from_line_iter(input);

        let starts = grid.find('S');
        if starts.len() != 1 {
            panic!("too few/many starts");
        }
        let start = starts.into_iter().next().unwrap();

        let ends = grid.find('E');
        if ends.len() != 1 {
            panic!("too few/many ends");
        }
        let end = ends.into_iter().next().unwrap();

        grid.set('.', start.row, start.col);
        grid.set('.', end.row, end.col);

        Self { grid, start, end }
    }
}

impl DijkstraSearchable for &Map {
    type Node = Coord2D<usize>;
    type Cost = usize;

    fn neighbors(
        &self,
        previous: &Coord2D<usize>,
        previous_cost: usize,
    ) -> Vec<(Coord2D<usize>, usize)> {
        previous
            .cardinal_neighbors()
            .into_iter()
            .filter(|x| {
                x.row < self.grid.n_rows
                    && x.col < self.grid.n_cols
                    && self.grid.get(x).unwrap() != '#'
            })
            .map(|x| (x, previous_cost + 1))
            .collect()
    }
}
