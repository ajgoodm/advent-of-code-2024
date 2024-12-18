use std::collections::HashSet;

use rayon::prelude::*;

use coord_2d::Coord2D;
use utils::{parse_iter, shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_18/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_18/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let map = parse_input_part_1(input, 1024, 71, 71);
    println!("part 1: {}", part_1_inner(map));
}

fn part_1_inner(map: MapPart1) -> usize {
    let end = Coord2D::new(map.n_rows - 1, map.n_cols - 1);
    shortest_path_length::<Coord2D<usize>, usize, MapPart1>(
        map,
        Coord2D::new(0, 0),
        HashSet::from([end]),
    )
    .unwrap()
}

fn part_2(input: AocBufReader) {
    let bytes = parse_bytes(input);
    println!("part 2: {:?}", part_2_inner(bytes, 71, 71))
}

fn part_2_inner(bytes: Vec<Coord2D<usize>>, n_rows: usize, n_cols: usize) -> Coord2D<usize> {
    let end = Coord2D::new(n_rows - 1, n_cols - 1);
    let candidate_byte_indices = (1024..bytes.len()).collect::<Vec<_>>();
    let blocking_byte_idx = candidate_byte_indices
        .par_iter()
        .find_first(|&nth_byte_idx| {
            let mut n_bytes = bytes.clone();
            n_bytes.truncate(*nth_byte_idx);
            let n_bytes: HashSet<Coord2D<usize>> = n_bytes.into_iter().collect();
            let map = MapPart1::new(n_bytes, n_rows, n_cols);
            let shortest_path = shortest_path_length::<Coord2D<usize>, usize, MapPart1>(
                map,
                Coord2D::new(0, 0),
                HashSet::from([end.clone()]),
            );

            shortest_path.is_none()
        })
        .unwrap();

    bytes[*blocking_byte_idx - 1].clone()
}

fn parse_bytes(input: AocBufReader) -> Vec<Coord2D<usize>> {
    parse_iter::<Coord2D<usize>, String>(input)
        .map(|x| Coord2D::new(x.col, x.row))
        .collect()
}

fn parse_input_part_1(
    input: AocBufReader,
    n_bytes: usize,
    n_rows: usize,
    n_cols: usize,
) -> MapPart1 {
    let mut bytes = parse_bytes(input);
    bytes.truncate(n_bytes);
    let bytes: HashSet<Coord2D<usize>> = bytes.into_iter().collect();

    MapPart1::new(bytes, n_rows, n_cols)
}

struct MapPart1 {
    bytes: HashSet<Coord2D<usize>>,
    n_rows: usize,
    n_cols: usize,
}

impl MapPart1 {
    fn new(bytes: HashSet<Coord2D<usize>>, n_rows: usize, n_cols: usize) -> Self {
        Self {
            bytes,
            n_rows,
            n_cols,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row_idx in 0..self.n_rows {
            let row = (0..self.n_cols)
                .map(
                    |col_idx| match self.bytes.contains(&Coord2D::new(row_idx, col_idx)) {
                        true => '#',
                        false => '.',
                    },
                )
                .collect::<String>();
            println!("{}", row);
        }
    }
}

impl DijkstraSearchable for MapPart1 {
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
            .filter(|neighbor| {
                neighbor.row < self.n_rows
                    && neighbor.col < self.n_cols
                    && !self.bytes.contains(neighbor)
            })
            .map(|neighbor| (neighbor, previous_cost + 1))
            .collect()
    }
}
