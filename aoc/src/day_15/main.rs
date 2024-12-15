use std::collections::HashSet;

use coord_2d::Coord2D;
use direction::CardinalDirection;
use grid::Grid;
use itertools::Itertools;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_15/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_15/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let warehouse = Warehouse::from_input_part_1(input);
    println!("part 1: {}", part_1_inner(warehouse))
}

fn part_1_inner(mut warehouse: Warehouse) -> usize {
    warehouse.execute_part_1();
    warehouse.sum_gps_coords('O')
}

fn part_2(input: AocBufReader) {
    let warehouse = Warehouse::from_input_part_2(input);
    println!("part 2: {}", part_2_inner(warehouse))
}

fn part_2_inner(mut warehouse: Warehouse) -> usize {
    warehouse.execute_part_2();
    warehouse.sum_gps_coords('[')
}

struct Warehouse {
    robot_position: Coord2D<usize>,
    map: Grid<char>,
    directions: std::vec::IntoIter<CardinalDirection>,
}

impl Warehouse {
    fn execute_part_1(&mut self) {
        while let Some(direction) = self.directions.next() {
            self.move_robot_part_1(direction);
        }
    }

    fn move_robot_part_1(&mut self, direction: CardinalDirection) {
        let robot_position = self.robot_position.clone();
        let robot_dest = robot_position.adjacent(&direction).unwrap();
        let robot_dest_obj = self.map.get(&robot_dest).unwrap();

        match robot_dest_obj {
            '.' => {
                // empty space, just move the robot!
                self.swap(&self.robot_position.clone(), &robot_dest);
                self.robot_position = robot_dest;
            }
            '#' => {
                // it's a wall; stupid robot
            }
            'O' => {
                // let's try to push some boxes!
                let mut first_non_box_location = robot_dest.clone();
                let first_non_box: char = {
                    loop {
                        first_non_box_location =
                            first_non_box_location.adjacent(&direction).unwrap();
                        if self.map.get(&first_non_box_location).unwrap() != 'O' {
                            break;
                        }
                    }
                    self.map.get(&first_non_box_location).unwrap()
                };

                if first_non_box == '.' {
                    // we have an empty space, let's move those boxes!
                    self.swap(&robot_dest, &first_non_box_location);
                    self.swap(&robot_position, &robot_dest);
                    self.robot_position = robot_dest;
                } else if first_non_box == '#' {
                    // we can't push the boxes; a wall is in the way
                } else {
                    panic!("something went wrong moving the robot")
                }
            }
            _ => panic!("something went wrong moving the robot {}", robot_dest_obj),
        }
    }

    fn execute_part_2(&mut self) {
        while let Some(direction) = self.directions.next() {
            self.move_robot_part_2(direction);
        }
    }

    fn move_robot_part_2(&mut self, direction: CardinalDirection) {
        let robot_position = self.robot_position.clone();
        let robot_dest = robot_position.adjacent(&direction).unwrap();
        let robot_dest_obj = self.map.get(&robot_dest).unwrap();

        match robot_dest_obj {
            '.' => {
                // empty space, just move the robot!
                self.swap(&self.robot_position.clone(), &robot_dest);
                self.robot_position = robot_dest;
            }
            '#' => {
                // it's a wall; stupid robot
            }
            '[' | ']' => {
                // we will try to move some boxes
                match direction {
                    CardinalDirection::North | CardinalDirection::South => self
                        .move_north_or_south(direction, robot_position, robot_dest, robot_dest_obj),
                    CardinalDirection::East | CardinalDirection::West => self.move_east_or_west(
                        direction,
                        robot_position,
                        robot_dest,
                        robot_dest_obj,
                    ),
                }
            }
            _ => panic!("something went wrong moving the robot {}", robot_dest_obj),
        }
    }

    fn move_north_or_south(
        &mut self,
        direction: CardinalDirection,
        robot_position: Coord2D<usize>,
        robot_dest: Coord2D<usize>,
        robot_dest_obj: char,
    ) {
        let mut boxes_to_check: Vec<Coord2D<usize>> = Vec::new();
        // we'll just store their left edge for simplicity
        match robot_dest_obj {
            '[' => {
                boxes_to_check.push(robot_dest.clone());
            }
            ']' => {
                boxes_to_check.push(robot_dest.west().unwrap());
            }
            _ => panic!("I thought we were pushing boxes"),
        }

        let mut we_can_move_boxes: bool = true;
        let mut boxes_to_move: HashSet<Coord2D<usize>> = HashSet::new();
        while let Some(next_box_left_side) = boxes_to_check.pop() {
            let next_box_right_side = next_box_left_side.east();

            let location_above_left = next_box_left_side.adjacent(&direction).unwrap();
            let location_above_right = next_box_right_side.adjacent(&direction).unwrap();

            let char_above_left = self.map.get(&location_above_left).unwrap();
            let char_above_right = self.map.get(&location_above_right).unwrap();
            match char_above_left {
                '[' => {
                    // there's a box stacked directly above this box!
                    boxes_to_check.push(location_above_left);
                }
                ']' => {
                    // this box is pushing the right half a box above
                    boxes_to_check.push(location_above_left.west().unwrap());
                }
                '.' => {
                    // there is no box directly above the left half of this box;
                }
                '#' => {
                    // this box is pushing up against a wall; we won't be able to push boxes
                    we_can_move_boxes = false;
                    break;
                }
                _ => panic!(),
            }
            match char_above_right {
                '[' => {
                    // there's a box stacked directly above this box!
                    boxes_to_check.push(location_above_right);
                }
                ']' => {
                    // there's a box directly above this box; covered by the case in the match
                    // statement above
                }
                '.' => {
                    // there is no box directly above the left half of this box;
                }
                '#' => {
                    // this box is pushing up against a wall; we won't be able to push boxes
                    we_can_move_boxes = false;
                    break;
                }
                _ => panic!(),
            }
            boxes_to_move.insert(next_box_left_side);
        }

        let boxes_to_move: Vec<Coord2D<usize>> = match direction {
            CardinalDirection::North => boxes_to_move
                .into_iter()
                .sorted_by_key(|x| usize::MAX - x.row)
                .collect(),
            CardinalDirection::East => boxes_to_move.into_iter().sorted_by_key(|x| x.col).collect(),
            CardinalDirection::South => {
                boxes_to_move.into_iter().sorted_by_key(|x| x.row).collect()
            }
            CardinalDirection::West => boxes_to_move
                .into_iter()
                .sorted_by_key(|x| usize::MAX - x.col)
                .collect(),
        };

        if we_can_move_boxes {
            self.move_boxes(boxes_to_move, direction);
            self.swap(&robot_position, &robot_dest);
            self.robot_position = robot_dest;
        }
    }

    fn move_east_or_west(
        &mut self,
        direction: CardinalDirection,
        robot_position: Coord2D<usize>,
        robot_dest: Coord2D<usize>,
        robot_dest_obj: char,
    ) {
        let mut boxes_to_move: Vec<Coord2D<usize>> = Vec::new();
        let mut current_box = match robot_dest_obj {
            '[' => robot_dest.clone(),
            ']' => robot_dest.west().unwrap(),
            _ => panic!("I thought we were pushing boxes"),
        };

        let mut we_can_move_boxes: bool = true;
        loop {
            boxes_to_move.push(current_box.clone());
            let next_box = current_box
                .adjacent(&direction)
                .unwrap()
                .adjacent(&direction)
                .unwrap();
            let neighbor = match direction {
                CardinalDirection::East => next_box.clone(),
                CardinalDirection::West => current_box.west().unwrap(),
                _ => panic!(),
            };

            match self.map.get(&neighbor).unwrap() {
                '[' | ']' => {
                    // another box on the stack
                }
                '.' => {
                    // an empty space, let's move some boxes!
                    break;
                }
                '#' => {
                    // a wall!
                    we_can_move_boxes = false;
                    break;
                }
                _ => {
                    panic!();
                }
            }

            current_box = next_box;
        }

        if we_can_move_boxes {
            self.move_boxes(boxes_to_move, direction);
            self.swap(&robot_position, &robot_dest);
            self.robot_position = robot_dest;
        }
    }

    fn move_boxes(&mut self, mut boxes: Vec<Coord2D<usize>>, direction: CardinalDirection) {
        // we've added boxes in a stack, so we need to remove
        // them from the end and move them in order
        while let Some(left_side) = boxes.pop() {
            let right_side = left_side.east();

            match direction {
                CardinalDirection::North => {
                    self.swap(&left_side, &left_side.north().unwrap());
                    self.swap(&right_side, &right_side.north().unwrap());
                }
                CardinalDirection::East => {
                    self.swap(&right_side, &right_side.east());
                    self.swap(&left_side, &right_side);
                }
                CardinalDirection::South => {
                    self.swap(&left_side, &left_side.south());
                    self.swap(&right_side, &right_side.south());
                }
                CardinalDirection::West => {
                    self.swap(&left_side.west().unwrap(), &left_side);
                    self.swap(&left_side, &right_side);
                }
            }
        }
    }

    fn swap(&mut self, x: &Coord2D<usize>, y: &Coord2D<usize>) {
        let x_val = self.map.get(x).unwrap();
        let y_val = self.map.get(y).unwrap();
        self.map.set(y_val, x.row, x.col);
        self.map.set(x_val, y.row, y.col);
    }

    fn sum_gps_coords(&self, box_edge: char) -> usize {
        self.map
            .find(box_edge)
            .into_iter()
            .map(|x| 100 * x.row + x.col)
            .sum()
    }

    fn from_input_part_1(mut input: impl Iterator<Item = String>) -> Self {
        let mut grid_chars: Vec<String> = Vec::new();
        for line in input.by_ref() {
            if line == *"" {
                break;
            }
            grid_chars.push(line);
        }
        let grid = Grid::from_line_iter(grid_chars.into_iter());

        let mut directions: Vec<CardinalDirection> = Vec::new();
        for line in input {
            directions.extend(line.chars().map(CardinalDirection::from_char))
        }

        let robot_positions = grid.find('@');
        if robot_positions.len() != 1 {
            panic!("Too many robots! {}", robot_positions.len());
        }
        let robot_position = robot_positions.into_iter().next().unwrap();

        Self {
            robot_position,
            map: grid,
            directions: directions.into_iter(),
        }
    }

    fn from_input_part_2(mut input: impl Iterator<Item = String>) -> Self {
        let mut grid_chars: Vec<String> = Vec::new();
        for line in input.by_ref() {
            if line == *"" {
                break;
            }
            let mut doubled_line = String::new();
            for c in line.chars() {
                match c {
                    '#' => doubled_line.extend(['#', '#']),
                    'O' => doubled_line.extend(['[', ']']),
                    '.' => doubled_line.extend(['.', '.']),
                    '@' => doubled_line.extend(['@', '.']),
                    _ => panic!("there's no such thing as 2's"),
                }
            }

            grid_chars.push(doubled_line);
        }
        let grid = Grid::from_line_iter(grid_chars.into_iter());

        let mut directions: Vec<CardinalDirection> = Vec::new();
        for line in input {
            directions.extend(line.chars().map(CardinalDirection::from_char))
        }

        let robot_positions = grid.find('@');
        if robot_positions.len() != 1 {
            panic!("Too many robots! {}", robot_positions.len());
        }
        let robot_position = robot_positions.into_iter().next().unwrap();

        Self {
            robot_position,
            map: grid,
            directions: directions.into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_gps_coords_part_1() {
        let warehouse = Warehouse::from_input_part_1(
            [
                "##########",
                "#.O.O.OOO#",
                "#........#",
                "#OO......#",
                "#OO@.....#",
                "#O#.....O#",
                "#O.....OO#",
                "#O.....OO#",
                "#OO....OO#",
                "##########",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(warehouse.sum_gps_coords('O'), 10092);
    }

    #[test]
    fn test_sub_gps_coords_part_2() {
        let warehouse = Warehouse {
            robot_position: Coord2D::new(0, 0),
            map: Grid::from_line_iter(
                [
                    "####################",
                    "##[].......[].[][]##",
                    "##[]...........[].##",
                    "##[]........[][][]##",
                    "##[]......[]....[]##",
                    "##..##......[]....##",
                    "##..[]............##",
                    "##..@......[].[][]##",
                    "##......[][]..[]..##",
                    "####################",
                ]
                .into_iter()
                .map(|x| x.to_string()),
            ),
            directions: vec![].into_iter(),
        };
        assert_eq!(warehouse.sum_gps_coords('['), 9021)
    }
}
