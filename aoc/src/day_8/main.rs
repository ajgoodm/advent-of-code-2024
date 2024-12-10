use std::collections::{HashMap, HashSet};

use coord_2d::Coord2D;
use grid::{char_grid_from_line, Grid};
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_8/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_8/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid = char_grid_from_line(input);
    println!("{}", part_1_inner(grid))
}

fn part_2(input: AocBufReader) {
    let grid = char_grid_from_line(input);
    println!("{}", part_2_inner(grid))
}

fn part_1_inner(grid: Grid<char>) -> usize {
    let mut char_positions: HashMap<char, Vec<Coord2D<isize>>> = HashMap::new();
    for (coord, c) in grid.coords_and_vals::<isize>().filter(|&(_, c)| c != '.') {
        char_positions.entry(c).or_default().push(coord);
    }

    char_positions
        .into_iter()
        .flat_map(|(_, antennae_loc)| {
            let mut nodes: Vec<Coord2D<isize>> = Vec::new();
            let n_locations = antennae_loc.len();
            for first_idx in 0..n_locations {
                for second_idx in (first_idx + 1)..n_locations {
                    let first = &antennae_loc[first_idx];
                    let second = &antennae_loc[second_idx];
                    let v = first.clone() - second.clone();

                    let candidate_1 = first.clone() + v.clone();
                    let candidate_2 = second.clone() - v;
                    if candidate_1.is_nonnegative() && grid.get(&candidate_1).is_some() {
                        nodes.push(candidate_1);
                    }

                    if candidate_2.is_nonnegative() && grid.get(&candidate_2).is_some() {
                        nodes.push(candidate_2);
                    }
                }
            }
            nodes
        })
        .collect::<HashSet<Coord2D<isize>>>()
        .len()
}

fn part_2_inner(grid: Grid<char>) -> usize {
    let mut char_positions: HashMap<char, Vec<Coord2D<isize>>> = HashMap::new();
    for (coord, c) in grid.coords_and_vals::<isize>().filter(|&(_, c)| c != '.') {
        char_positions.entry(c).or_default().push(coord);
    }

    char_positions
        .into_iter()
        .flat_map(|(_, antennae_loc)| {
            let mut nodes: Vec<Coord2D<isize>> = Vec::new();
            let n_locations = antennae_loc.len();
            for first_idx in 0..n_locations {
                for second_idx in (first_idx + 1)..n_locations {
                    let first = &antennae_loc[first_idx];
                    let second = &antennae_loc[second_idx];
                    let v = first.clone() - second.clone();

                    let mut harmonic = 0isize;
                    loop {
                        let candidate = first.clone() + v.clone().mul_scalar(harmonic);
                        if candidate.row < 0
                            || candidate.col < 0
                            || candidate.row >= grid.n_rows as isize
                            || candidate.col >= grid.n_cols as isize
                        {
                            break;
                        }
                        if grid.get(&candidate).is_some() {
                            nodes.push(candidate);
                        }
                        harmonic += 1;
                    }

                    let mut harmonic = 0isize;
                    loop {
                        let candidate = second.clone() - v.clone().mul_scalar(harmonic);
                        if candidate.row < 0
                            || candidate.col < 0
                            || candidate.row >= grid.n_rows as isize
                            || candidate.col >= grid.n_cols as isize
                        {
                            break;
                        }
                        if grid.get(&candidate).is_some() {
                            nodes.push(candidate);
                        }
                        harmonic += 1;
                    }
                }
            }
            nodes
        })
        .collect::<HashSet<Coord2D<isize>>>()
        .len()
}
