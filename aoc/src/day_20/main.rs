use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use coord_2d::Coord2D;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_20/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_20/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let map = Map::from_input(input);
    println!("part 1: {}", inner(map, 2));
}

fn part_2(input: AocBufReader) {
    let map = Map::from_input(input);
    println!("part 2: {}", inner(map, 20));
}

type SourceDest = (Coord2D<usize>, Coord2D<usize>);
type Cache = HashMap<SourceDest, Option<usize>>;

/// We can consider every barrier position and determine the minimum path
/// length from each side of the barrier. If that length is > 100, then
/// we can consider that barrier position a candidate to check later.
fn inner(map: Map, cheat_len: usize) -> usize {
    let open_coords: Vec<_> = map.grid.find('.').into_iter().collect();

    // build a list of start, end pairs from the start to every other coordinate
    // and from every coordinate to the end
    let mut src_dest_pairs: Vec<_> = open_coords
        .iter()
        .map(|e| (map.start.clone(), e.clone()))
        .collect();
    src_dest_pairs.extend(open_coords.iter().map(|s| (s.clone(), map.end.clone())));

    // build up a cache mapping the shortest path from the start to
    // every coordinate and from every coordinate to the end
    let mut cache: Cache = HashMap::new();
    for src_dest in src_dest_pairs {
        let mut visited: HashSet<Coord2D<usize>> = HashSet::new();
        let result = shortest_path(&map, &src_dest, &mut visited, &cache);
        cache.insert(src_dest.clone(), result);
    }

    let original_length = cache
        .get(&(map.start.clone(), map.end.clone()))
        .unwrap()
        .unwrap();
    let cheat_start_ends = map.cheats(cheat_len);
    cheat_start_ends
        .into_iter()
        .filter(|(cheat_start, cheat_end)| {
            if let (Some(Some(start_to_cheat)), Some(Some(cheat_to_end))) = (
                cache.get(&(map.start.clone(), cheat_start.clone())),
                cache.get(&(cheat_end.clone(), map.end.clone())),
            ) {
                let length_w_cheat =
                    start_to_cheat + cheat_start.manhattan_distance(cheat_end) + cheat_to_end;
                length_w_cheat < original_length && (original_length - length_w_cheat) >= 100
            } else {
                false
            }
        })
        .count()
}

fn shortest_path(
    map: &Map,
    start_end: &(Coord2D<usize>, Coord2D<usize>),
    visited: &mut HashSet<Coord2D<usize>>,
    cache: &Cache,
) -> Option<usize> {
    let start = &start_end.0;
    let end = &start_end.1;
    if start == end {
        return Some(0);
    }

    if let Some(cached) = cache.get(start_end) {
        return *cached;
    }

    visited.insert(start.clone());

    let next: Vec<_> = start
        .cardinal_neighbors()
        .into_iter()
        .filter(|x| {
            !visited.contains(x)
                && x.row < map.grid.n_rows
                && x.col < map.grid.n_cols
                && map.grid.get(x).unwrap() != '#'
        })
        .collect();

    next.into_iter()
        .filter_map(|x| shortest_path(map, &(x.clone(), end.clone()), visited, cache))
        .min()
        .map(|x| x + 1)
}

struct Map {
    grid: Grid<char>,
    start: Coord2D<usize>,
    end: Coord2D<usize>,
}

impl Map {
    /// Return every pair of coords separated (manhattan distance)
    /// by up to len that are both open spaces
    fn cheats(&self, len: usize) -> Vec<(Coord2D<usize>, Coord2D<usize>)> {
        let open_spaces: Vec<_> = self.grid.find('.').into_iter().collect();
        let n_open_spaces = open_spaces.len();
        (0..n_open_spaces)
            .cartesian_product(0..n_open_spaces)
            .filter(|(first_idx, second_idx)| {
                if first_idx == second_idx {
                    false
                } else {
                    let first = &open_spaces[*first_idx];
                    let second = &open_spaces[*second_idx];
                    first.manhattan_distance(second) <= len
                }
            })
            .map(|(first_idx, second_idx)| {
                (
                    open_spaces[first_idx].clone(),
                    open_spaces[second_idx].clone(),
                )
            })
            .collect()
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
