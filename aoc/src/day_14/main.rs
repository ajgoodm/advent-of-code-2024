use std::str::FromStr;
use std::sync::LazyLock;
use std::{collections::HashMap, io::Write};

use regex::Regex;

use coord_2d::Coord2D;
use utils::{parse_iter, AocBufReader};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_14/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_14/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let robots: Vec<Robot> = parse_iter(input).collect();
    let map = Map::new(101, 103, robots);

    println!("part 1: {}", part_1_inner(map))
}

fn part_2(input: AocBufReader) {
    let robots: Vec<Robot> = parse_iter(input).collect();
    let mut map = Map::new(101, 103, robots);

    let mut f = std::fs::File::create("/tmp/foo").expect("Unable to create file");
    for n_steps in 0..10000 {
        f.write_all(format!("\n*** n_steps: {}\n", n_steps).as_bytes())
            .unwrap();
        map.print_to_file(&mut f);
        map.step();
    }
}

fn part_1_inner(mut map: Map) -> usize {
    for _ in 0..100 {
        map.step();
    }
    map.safety_factor()
}

struct Map {
    n_x: isize,
    n_y: isize,
    robots: Vec<Robot>,
}

impl Map {
    fn new(n_x: isize, n_y: isize, robots: Vec<Robot>) -> Self {
        Self { n_x, n_y, robots }
    }

    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.step(self.n_x, self.n_y);
        }
    }

    fn safety_factor(&self) -> usize {
        let mut quadrant_1_ct: usize = 0;
        let mut quadrant_2_ct: usize = 0;
        let mut quadrant_3_ct: usize = 0;
        let mut quadrant_4_ct: usize = 0;

        for robot in self.robots.iter() {
            if robot.position.col == self.n_x / 2 || robot.position.row == self.n_y / 2 {
                continue;
            }

            match (
                robot.position.col > self.n_x / 2,
                robot.position.row > self.n_y / 2,
            ) {
                (true, true) => quadrant_1_ct += 1,
                (true, false) => quadrant_2_ct += 1,
                (false, true) => quadrant_3_ct += 1,
                (false, false) => quadrant_4_ct += 1,
            }
        }

        quadrant_1_ct * quadrant_2_ct * quadrant_3_ct * quadrant_4_ct
    }

    fn as_string(&self) -> String {
        let mut robot_position_cts: HashMap<&Coord2D<isize>, usize> = HashMap::new();
        for robot in self.robots.iter() {
            *robot_position_cts.entry(&robot.position).or_insert(0) += 1;
        }

        let mut rows: Vec<String> = vec![];
        for row_idx in 0..self.n_y {
            let row = (0..self.n_x)
                .map(
                    |col_idx| match robot_position_cts.get(&Coord2D::new(row_idx, col_idx)) {
                        Some(val) => format!("{}", val),
                        None => ".".to_string(),
                    },
                )
                .collect::<String>();
            rows.push(row);
        }

        rows.join("\n")
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("\n");
        std::io::stdout()
            .write_all(self.as_string().as_bytes())
            .unwrap();
        println!("\n");
    }

    fn print_to_file(&self, f: &mut std::fs::File) {
        f.write_all(self.as_string().as_bytes()).unwrap();
    }
}

#[derive(Debug)]
struct Robot {
    position: Coord2D<isize>,
    velocity: Coord2D<isize>,
}

fn wrapped_add(a: isize, b: isize, bound: isize) -> isize {
    let mut sum = a + b;
    while sum < 0 {
        sum += bound;
    }
    while sum >= bound {
        sum -= bound;
    }
    sum
}

impl Robot {
    fn new(p_x: isize, p_y: isize, v_x: isize, v_y: isize) -> Self {
        Self {
            position: Coord2D::new(p_y, p_x),
            velocity: Coord2D::new(v_y, v_x),
        }
    }

    fn step(&mut self, n_x: isize, n_y: isize) {
        self.position.col = wrapped_add(self.position.col, self.velocity.col, n_x);
        self.position.row = wrapped_add(self.position.row, self.velocity.row, n_y);
    }
}

static ROBOT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(?<px>\d*),(?<py>\d*) v=(?<vx>-?\d*),(?<vy>-?\d*)").unwrap());

#[derive(Debug, PartialEq, Eq)]
struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ROBOT_REGEX.captures(s).unwrap();
        Ok(Self::new(
            captures["px"].parse().unwrap(),
            captures["py"].parse().unwrap(),
            captures["vx"].parse().unwrap(),
            captures["vy"].parse().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let robots = parse_iter::<Robot, &str>(
            [
                "p=0,4 v=3,-3",
                "p=6,3 v=-1,-3",
                "p=10,3 v=-1,2",
                "p=2,0 v=2,-1",
                "p=0,0 v=1,3",
                "p=3,0 v=-2,-2",
                "p=7,6 v=-1,-3",
                "p=3,0 v=-1,-2",
                "p=9,3 v=2,3",
                "p=7,3 v=-1,2",
                "p=2,4 v=2,-3",
                "p=9,5 v=-3,-3",
            ]
            .into_iter(),
        )
        .collect::<Vec<_>>();
        let map = Map::new(11, 7, robots);
        assert_eq!(part_1_inner(map), 12);
    }
}
