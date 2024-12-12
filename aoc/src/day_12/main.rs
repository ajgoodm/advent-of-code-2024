use std::collections::HashSet;

use coord_2d::Coord2D;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_12/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_12/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let grid = Grid::from_line_iter(input);
    println!("part 1: {}", part_1_inner(grid))
}

fn part_1_inner(grid: Grid<char>) -> usize {
    let map = Garden::new(grid);
    map.regions()
        .into_iter()
        .map(|region| region.cost_part_1())
        .sum()
}

fn part_2(input: AocBufReader) {
    let grid = Grid::from_line_iter(input);
    println!("part 1: {}", part_2_inner(grid))
}

fn part_2_inner(grid: Grid<char>) -> usize {
    let map = Garden::new(grid);
    map.regions()
        .into_iter()
        .map(|region| region.cost_part_2())
        .sum()
}

#[allow(dead_code)]
#[derive(Debug)]
struct Region {
    plant_type: char,
    coords: HashSet<Coord2D<usize>>,
}

impl Region {
    fn new(plant_type: char, coords: HashSet<Coord2D<usize>>) -> Self {
        Self { plant_type, coords }
    }

    /// The number of sides is equal to the number
    /// of times line dividing the region for the external
    /// area changes directions. Count the number of corners
    /// in this imaginary line bounding the region
    fn n_sides(&self) -> usize {
        self.coords
            .iter()
            .cloned()
            .map(|member| {
                let n_touching = [
                    member.north(),
                    Some(member.east()),
                    Some(member.south()),
                    member.west(),
                ]
                .into_iter()
                .filter(|neighbor| match neighbor {
                    Some(x) => self.coords.contains(x),
                    None => false,
                })
                .count();

                match n_touching {
                    0 => 4, // this is a region of area 1 (4 corners)
                    1 => 2, // this block is a "finger" - and contributes 2 corners
                    2 => self.touch_2_corners(&member),
                    3 => self.touch_3_corners(&member),
                    4 => self.touch_4_corners(&member),
                    _ => panic!(),
                }
            })
            .sum()
    }

    fn touch_2_corners(&self, coord: &Coord2D<usize>) -> usize {
        let touches_north = match coord.north() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let touches_east = self.coords.contains(&coord.east());
        let touches_south = self.coords.contains(&coord.south());
        let touches_west = match coord.west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };

        let is_straight = (touches_east && touches_west) || (touches_north && touches_south);
        if is_straight {
            return 0;
        }

        let corner_filled = {
            if touches_east && touches_south {
                self.coords.contains(&coord.south_east())
            } else if touches_south && touches_west {
                match coord.south_west() {
                    Some(c) => self.coords.contains(&c),
                    None => false,
                }
            } else if touches_west && touches_north {
                match coord.north_west() {
                    Some(c) => self.coords.contains(&c),
                    None => false,
                }
            } else if touches_north && touches_east {
                match coord.north_east() {
                    Some(c) => self.coords.contains(&c),
                    None => false,
                }
            } else {
                panic!()
            }
        };
        if corner_filled {
            1
        } else {
            2
        }
    }

    fn touch_3_corners(&self, coord: &Coord2D<usize>) -> usize {
        let touches_north = match coord.north() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let touches_east = self.coords.contains(&coord.east());
        let touches_south = self.coords.contains(&coord.south());
        let touches_west = match coord.west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };

        let ne_full = match coord.north_east() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let se_full = self.coords.contains(&coord.south_east());
        let sw_full = match coord.south_west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let nw_full = match coord.north_west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };

        let n_corners_filled: usize = if !touches_north {
            [sw_full, se_full].into_iter().filter(|x| *x).count()
        } else if !touches_east {
            [nw_full, sw_full].into_iter().filter(|x| *x).count()
        } else if !touches_south {
            [nw_full, ne_full].into_iter().filter(|x| *x).count()
        } else if !touches_west {
            [ne_full, se_full].into_iter().filter(|x| *x).count()
        } else {
            panic!()
        };

        match n_corners_filled {
            2 => 0,
            1 => 1,
            0 => 2,
            _ => panic!(),
        }
    }

    fn touch_4_corners(&self, coord: &Coord2D<usize>) -> usize {
        let ne_full = match coord.north_east() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let se_full = self.coords.contains(&coord.south_east());
        let sw_full = match coord.south_west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };
        let nw_full = match coord.north_west() {
            Some(c) => self.coords.contains(&c),
            None => false,
        };

        4 - [ne_full, se_full, sw_full, nw_full]
            .into_iter()
            .filter(|x| *x)
            .count()
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimiter_length(&self) -> usize {
        self.coords
            .iter()
            .cloned()
            .map(|coord| {
                [
                    coord.north(),
                    Some(coord.east()),
                    Some(coord.south()),
                    coord.west(),
                ]
                .into_iter()
                .filter(|neighbor| match neighbor {
                    None => true,
                    Some(n) => !self.coords.contains(n),
                })
                .count()
            })
            .sum()
    }

    fn cost_part_1(&self) -> usize {
        self.area() * self.perimiter_length()
    }

    fn cost_part_2(&self) -> usize {
        self.area() * self.n_sides()
    }
}

struct Garden {
    map: Grid<char>,
}

impl Garden {
    fn new(map: Grid<char>) -> Self {
        Self { map }
    }

    fn regions(&self) -> Vec<Region> {
        // the coords we've already allocated to a region
        let mut seen: HashSet<Coord2D<usize>> = HashSet::new();

        let mut result: Vec<Region> = Vec::new();
        for (coord, plant_type) in self.map.coords_and_vals::<usize>() {
            if seen.contains(&coord) {
                continue;
            }

            let mut this_region: HashSet<Coord2D<usize>> = HashSet::new();
            let mut to_visit: HashSet<Coord2D<usize>> = HashSet::from([coord]);
            while !to_visit.is_empty() {
                let next = to_visit.iter().next().cloned().unwrap();
                to_visit.remove(&next);

                this_region.insert(next.clone());
                seen.insert(next.clone());

                to_visit.extend(next.cardinal_neighbors().into_iter().filter(|neighbor| {
                    let is_correct_char = match self.map.get(neighbor) {
                        Some(c) => c == plant_type,
                        None => false,
                    };

                    let seen_before = seen.contains(neighbor);
                    is_correct_char && !seen_before
                }));
            }

            result.push(Region::new(plant_type, this_region));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let grid = Grid::from_line_iter(
            ["AAAA", "BBCD", "BBCC", "EEEC"]
                .into_iter()
                .map(|x| x.to_string()),
        );
        assert_eq!(part_1_inner(grid), 140)
    }

    #[test]
    fn test_part_1_holes() {
        let grid = Grid::from_line_iter(
            ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"]
                .into_iter()
                .map(|x| x.to_string()),
        );
        assert_eq!(part_1_inner(grid), 772)
    }

    // #[test]
    // fn test_part_2() {
    //     let grid = Grid::from_line_iter([
    //         "EEEEE",
    //         "EXXXX",
    //         "EEEEE",
    //         "EXXXX",
    //         "EEEEE",
    //     ].into_iter().map(|x| x.to_string()));
    //     assert_eq!(
    //         part_2_inner(grid),
    //         236
    //     )
    // }

    #[test]
    fn test_part_2_second() {
        let grid = Grid::from_line_iter(
            ["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"]
                .into_iter()
                .map(|x| x.to_string()),
        );
        assert_eq!(part_2_inner(grid), 368)
    }
}
