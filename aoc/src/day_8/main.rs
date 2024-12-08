use std::collections::{HashMap, HashSet};

use coord_2d::Coord2D;
use grid::{from_line_iter, Grid};
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_8/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid: Grid<char> = from_line_iter(input);
    println!("{}", part_1_inner(grid))
}

fn part_1_inner(grid: Grid<char>) -> usize {
    let mut char_positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (coord, c) in grid.coords_and_vals().filter(|&(_, c)| c != '.') {
        char_positions.entry(c).or_default().push((
            isize::try_from(coord.row).unwrap(),
            isize::try_from(coord.col).unwrap(),
        ));
    }

    char_positions
        .into_iter()
        .flat_map(|(_, antennae_loc)| {
            let mut nodes: Vec<Coord2D<usize>> = Vec::new();
            let n_locations = antennae_loc.len();
            for first_idx in 0..n_locations {
                for second_idx in (first_idx + 1)..n_locations {
                    let first = &antennae_loc[first_idx];
                    let second = &antennae_loc[second_idx];
                    let v: (isize, isize) = (first.0 - second.0, first.1 - second.1);

                    let candidate_1 = (first.0 + v.0, first.1 + v.1);
                    let candidate_2 = (second.0 - v.0, second.1 - v.1);
                    if candidate_1.0 >= 0 && candidate_1.1 >= 0 {
                        let point = Coord2D::new(
                            usize::try_from(candidate_1.0).unwrap(),
                            usize::try_from(candidate_1.1).unwrap(),
                        );
                        if grid.get(&point).is_some() {
                            nodes.push(point);
                        }
                    }

                    if candidate_2.0 >= 0 && candidate_2.1 >= 0 {
                        let point = Coord2D::new(
                            usize::try_from(candidate_2.0).unwrap(),
                            usize::try_from(candidate_2.1).unwrap(),
                        );
                        if grid.get(&point).is_some() {
                            nodes.push(point);
                        }
                    }
                }
            }
            nodes
        })
        .collect::<HashSet<Coord2D<usize>>>()
        .len()
}
