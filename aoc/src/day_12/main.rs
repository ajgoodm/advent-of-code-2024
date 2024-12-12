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

    /// The number of sides is equal to the number of times the line dividing
    /// the region from the external area changes directions (the number of corners).
    ///
    /// This function counts the number of corners in this imaginary line bounding the region.
    /// It does this by counting the number of corners touched by each garden plot.
    /// The trick is to not overcount corners...
    /// This turns out to be very annoying
    ///
    /// For instance, if a garden plot touches one neighbor, then it touches 2 corners:
    ///
    /// [ ] [ ]|
    ///        +---+ <--
    /// [ ] [^] [*]|      we say this garden plot has two corners
    ///        +---+ <--  (in our counting scheme, the other two corners (+) belong to the block
    /// [ ] [ ]|          labelled ^, which is touching 4 neighbors
    ///
    /// It goes on laboriously for the case where the garden plot has 2, 3, or 4 neighbors;
    /// each case getting its own function called by this one. We sum up the corners,
    /// and we get the number of sides of the perimeter
    fn n_corners(&self) -> usize {
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
                    2 => self.count_corners_touch_2(&member),
                    3 => self.count_corners_touch_3(&member),
                    4 => self.count_corners_touch_4(&member),
                    _ => panic!(),
                }
            })
            .sum()
    }

    /// Garden plots that are touching 2 neighbors can be put in 3 categories (up to rotation):
    ///
    /// A)
    /// -----------
    /// [ ] [*] [ ]  <-- the 2 neighbors of the starred garden plot are in a straight line
    /// -----------      the starred plot has no corners
    ///
    /// B)
    /// -------+
    /// [ ] [*]|     <-- the 2 neighbors of the starred garden plot make a right angle;
    /// [ ] [ ]|         the interior of the angle is filled; the starred plot has one corner
    ///
    /// C)
    /// -------+
    /// [ ] [*]|     <-- the 2 neighbors of the starred garden plot make a right angle;
    /// ---+   |         the interior of the angle is empty; the starred plot has two corners
    ///    |[ ]|
    fn count_corners_touch_2(&self, coord: &Coord2D<usize>) -> usize {
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
            // case A in the docstring
            return 0;
        }

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

        let corner_filled = {
            if touches_east && touches_south {
                se_full
            } else if touches_south && touches_west {
                sw_full
            } else if touches_west && touches_north {
                nw_full
            } else if touches_north && touches_east {
                ne_full
            } else {
                panic!("This isn't realy an el-shaped corner 😬");
            }
        };
        if corner_filled {
            // case B in the docstring
            1
        } else {
            // case C in the docstring
            2
        }
    }

    /// Garden plots that are touching 3 neighbors can be put in 3 categories (up to rotation):
    ///
    /// [ ] [*] [ ]  <-- The three classes are defined by whether the coordinates
    /// ---+   +---      indicated by the question marks are filled (0, 1, or both)
    ///  ? |[ ]| ?       0 => 2 corners, 1 => 1 corner, both => zero corners (this is an edge)
    ///    +---+
    ///                  Note that we're kind of neglecting what is on the north side of the
    ///                  starred plot. We've taken care (sort of, we just got lucky) to make
    ///                  sure the other cases are counted by other rules
    fn count_corners_touch_3(&self, coord: &Coord2D<usize>) -> usize {
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

    /// If a plot touches 4 neighbors, the number of corners it creates
    /// is equal to the number of diagonal coordinates that are unoccupied.
    fn count_corners_touch_4(&self, coord: &Coord2D<usize>) -> usize {
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
        // the number of sides is equal to the number of corners
        self.area() * self.n_corners()
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

    #[test]
    fn test_part_2() {
        let grid = Grid::from_line_iter(
            ["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
                .into_iter()
                .map(|x| x.to_string()),
        );
        assert_eq!(part_2_inner(grid), 236)
    }

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
