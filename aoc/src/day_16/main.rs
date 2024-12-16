use std::collections::HashSet;

use coord_2d::Coord2D;
use direction::CardinalDirection;
use grid::Grid;
use utils::{shortest_path, AocBufReader, DijkstraSearchable};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_16/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (map, start, end) = parse_input(input);

    println!(
        "part 1: {}",
        part_1_inner(map, (start, CardinalDirection::East), end)
    );
}

fn part_1_inner(
    map: Map,
    start: (Coord2D<usize>, CardinalDirection),
    end_coord: Coord2D<usize>,
) -> usize {
    let ends: HashSet<(Coord2D<usize>, CardinalDirection)> = HashSet::from([
        (end_coord.clone(), CardinalDirection::North),
        (end_coord.clone(), CardinalDirection::East),
        (end_coord.clone(), CardinalDirection::South),
        (end_coord.clone(), CardinalDirection::West),
    ]);

    shortest_path::<(Coord2D<usize>, CardinalDirection), usize, Map>(map, start, ends).unwrap()
}

struct Map {
    grid: Grid<char>,
}

impl DijkstraSearchable for Map {
    type Node = (Coord2D<usize>, CardinalDirection);
    type Cost = usize;

    fn neighbors(
        &self,
        previous: &(Coord2D<usize>, CardinalDirection),
        previous_cost: usize,
    ) -> Vec<((Coord2D<usize>, CardinalDirection), usize)> {
        let mut result: Vec<((Coord2D<usize>, CardinalDirection), usize)> = Vec::new();
        let (previous_coord, previous_direction) = previous;
        result.push((
            (previous_coord.clone(), previous_direction.turn_left()),
            previous_cost + 1000,
        ));
        result.push((
            (previous_coord.clone(), previous_direction.turn_right()),
            previous_cost + 1000,
        ));

        let next_space = previous_coord.adjacent(previous_direction).unwrap();
        if let Some('.') = self.grid.get(&next_space) {
            result.push(((next_space, *previous_direction), previous_cost + 1));
        }

        result
    }
}

fn parse_input(input: AocBufReader) -> (Map, Coord2D<usize>, Coord2D<usize>) {
    let mut grid = Grid::from_line_iter(input);
    let start_coords = grid.find('S');
    if start_coords.len() != 1 {
        panic!("too many starts");
    }
    let start_coord = start_coords.into_iter().next().unwrap();

    let end_coords = grid.find('E');
    if end_coords.len() != 1 {
        panic!("too many ends");
    }
    let end_coord = end_coords.into_iter().next().unwrap();

    grid.set('.', start_coord.row, start_coord.col);
    grid.set('.', end_coord.row, end_coord.col);

    (Map { grid }, start_coord, end_coord)
}
