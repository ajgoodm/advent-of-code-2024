use coord_2d::Coord2D;
use direction::CardinalDirection;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_15/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let warehouse = Warehouse::from_input(input);
    println!("part 1: {}", part_1_inner(warehouse))
}

fn part_1_inner(mut warehouse: Warehouse) -> usize {
    warehouse.execute();
    warehouse.sum_gps_coords()
}

struct Warehouse {
    robot_position: Coord2D<usize>,
    map: Grid<char>,
    directions: std::vec::IntoIter<CardinalDirection>,
}

impl Warehouse {
    fn execute(&mut self) {
        while let Some(direction) = self.directions.next() {
            self.move_robot(direction);
        }
    }

    fn move_robot(&mut self, direction: CardinalDirection) {
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

    fn swap(&mut self, x: &Coord2D<usize>, y: &Coord2D<usize>) {
        let x_val = self.map.get(x).unwrap();
        let y_val = self.map.get(y).unwrap();
        self.map.set(y_val, x.row, x.col);
        self.map.set(x_val, y.row, y.col);
    }

    fn sum_gps_coords(&self) -> usize {
        self.map
            .find('O')
            .into_iter()
            .map(|x| 100 * x.row + x.col)
            .sum()
    }

    fn from_input(mut input: impl Iterator<Item = String>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_gps_coords() {
        let warehouse = Warehouse::from_input(
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
        assert_eq!(warehouse.sum_gps_coords(), 10092);
    }
}
