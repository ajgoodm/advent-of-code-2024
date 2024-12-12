use std::collections::{HashMap, HashSet};

use coord_2d::Coord2D;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid: Grid<isize> = Grid::from_line_iter(input).into_numeric_type::<isize>();
    println!("part 1: {}", part_1_inner(grid));
}

fn part_2(input: AocBufReader) {
    let grid: Grid<isize> = Grid::from_line_iter(input).into_numeric_type::<isize>();
    println!("part 1: {}", part_2_inner(grid));
}

fn part_1_inner(grid: Grid<isize>) -> usize {
    let mut map = TopographicMap::new(grid);
    let trail_heads: Vec<Coord2D<usize>> = map.trail_heads.clone();
    trail_heads
        .iter()
        .map(|trail_head| map.reachable_peaks(trail_head).len())
        .sum()
}

fn part_2_inner(grid: Grid<isize>) -> usize {
    let mut map = TopographicMap::new(grid);
    let trail_heads: Vec<Coord2D<usize>> = map.trail_heads.clone();
    trail_heads
        .iter()
        .map(|trail_head| map.distinct_trail_count(trail_head))
        .sum()
}

struct TopographicMap {
    grid: Grid<isize>,
    trail_heads: Vec<Coord2D<usize>>,
    cache_part_1: HashMap<Coord2D<usize>, HashSet<Coord2D<usize>>>, // which peaks are reachable from each point?
    cache_part_2: HashMap<Coord2D<usize>, usize>, // how many trails can be terminated from this point?
}

impl TopographicMap {
    fn new(grid: Grid<isize>) -> Self {
        let trail_heads = grid.find(0isize).into_iter().collect::<Vec<_>>();
        Self {
            grid,
            trail_heads,
            cache_part_1: HashMap::new(),
            cache_part_2: HashMap::new(),
        }
    }

    fn reachable_peaks(&mut self, start: &Coord2D<usize>) -> HashSet<Coord2D<usize>> {
        if let Some(cached) = self.cache_part_1.get(start) {
            return cached.clone();
        }

        let start_val = self.grid.get(start).unwrap();
        let neighbor_vals: HashMap<Coord2D<usize>, isize> = start
            .cardinal_neighbors()
            .into_iter()
            .filter_map(|neighbor| self.grid.get(&neighbor).map(|val| (neighbor, val)))
            .collect();

        if start_val == 8 {
            return neighbor_vals
                .into_iter()
                .filter(|(_, val)| *val == 9)
                .map(|(coord, _)| coord)
                .collect();
        }

        let reachable_neighbors: Vec<Coord2D<usize>> = neighbor_vals
            .into_iter()
            .filter(|(_, val)| *val == start_val + 1)
            .map(|(coord, _)| coord)
            .collect();
        let mut result: HashSet<Coord2D<usize>> = HashSet::new();
        for reachable_neighbor in reachable_neighbors {
            result.extend(self.reachable_peaks(&reachable_neighbor).clone());
        }

        self.cache_part_1.insert(start.clone(), result.clone());
        result
    }

    fn distinct_trail_count(&mut self, start: &Coord2D<usize>) -> usize {
        if let Some(count) = self.cache_part_2.get(start) {
            return *count;
        }

        let start_val = self.grid.get(start).unwrap();
        let neighbor_vals: HashMap<Coord2D<usize>, isize> = start
            .cardinal_neighbors()
            .into_iter()
            .filter_map(|neighbor| self.grid.get(&neighbor).map(|val| (neighbor, val)))
            .collect();

        if start_val == 8 {
            return neighbor_vals
                .into_iter()
                .filter(|(_, val)| *val == 9)
                .count();
        }

        let reachable_neighbors: Vec<Coord2D<usize>> = neighbor_vals
            .into_iter()
            .filter(|(_, val)| *val == start_val + 1)
            .map(|(coord, _)| coord)
            .collect();
        let mut result: usize = 0;
        for reachable_neighbor in reachable_neighbors {
            let n_paths = self.distinct_trail_count(&reachable_neighbor);
            result += n_paths;
        }

        self.cache_part_2.insert(start.clone(), result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let grid: Grid<isize> = Grid::from_line_iter(
            [
                "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
                "10456732",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        )
        .into_numeric_type::<isize>();
        assert_eq!(part_1_inner(grid), 36)
    }

    #[test]
    fn test_part_2() {
        let grid: Grid<isize> = Grid::from_line_iter(
            [
                "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
                "10456732",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        )
        .into_numeric_type::<isize>();
        assert_eq!(part_2_inner(grid), 81)
    }
}
