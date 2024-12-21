use std::collections::HashSet;

use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_21/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input));
}

fn part_1_inner(input: impl Iterator<Item = String>) -> usize {
    let mut result: usize = 0;
    for line in input {
        let shortest_sequence = shortest_sequence(&line[..], 2);
        let val = line[..(line.len() - 1)].parse::<usize>().unwrap();
        println!("shortest sequence: {}, val: {}", shortest_sequence, val);
        result += shortest_sequence * val
    }

    result
}

fn shortest_sequence(sequence: &str, n_operators: usize) -> usize {
    let numeric_key_presses: Vec<_> = sequence
        .chars()
        .map(NumericKeypadPosition::from_char)
        .collect();

    let operators: Vec<_> = (0..n_operators)
        .map(|_| DirectionalKeypadPosition::A)
        .collect();
    let start = State::new(operators.clone(), NumericKeypadPosition::A);

    let mut check_points: Vec<State> = vec![start];
    check_points.extend(
        numeric_key_presses
            .into_iter()
            .map(|x| State::new(operators.clone(), x)),
    );

    let mut n_pushes: usize = 0;
    for idx in 0..(check_points.len() - 1) {
        let from = &check_points[idx];
        let to = &check_points[idx + 1];
        n_pushes +=
            shortest_path_length(Me::new(), from.clone(), HashSet::from([to.clone()])).unwrap() + 1;
    }

    n_pushes
}

struct Me;

impl Me {
    fn new() -> Self {
        // happy birthday!
        Self {}
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    directional_pad_operators: Vec<DirectionalKeypadPosition>,
    numeric_pad_operator: NumericKeypadPosition,
}

impl State {
    fn new(
        directional_pad_operators: Vec<DirectionalKeypadPosition>,
        numeric_pad_operator: NumericKeypadPosition,
    ) -> Self {
        Self {
            directional_pad_operators,
            numeric_pad_operator,
        }
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, direction_keypad_input: DirectionalKeypadPosition) -> Option<State> {
        let mut input_for_next: DirectionalKeypadPosition = direction_keypad_input;

        for (ii, intermediate_operator) in self.directional_pad_operators.iter().enumerate() {
            if input_for_next == DirectionalKeypadPosition::A {
                input_for_next = intermediate_operator.clone();
            } else {
                match intermediate_operator
                    .neighbors()
                    .iter()
                    .find(|(_, input)| &input_for_next == input)
                {
                    Some((next, _)) => {
                        let operators = self
                            .directional_pad_operators
                            .iter()
                            .enumerate()
                            .map(|(jj, op)| if ii == jj { next.clone() } else { op.clone() })
                            .collect::<Vec<_>>();
                        return Some(State::new(operators, self.numeric_pad_operator.clone()));
                    }
                    None => return None,
                }
            }
        }

        if input_for_next == DirectionalKeypadPosition::A {
            // this doesn't do anything (we maybe pushed the wrong button)
            // this won't ever happen on the optimal route
            return Some(self.clone());
        } else {
            match self
                .numeric_pad_operator
                .neighbors()
                .iter()
                .find(|(_, input)| &input_for_next == input)
            {
                Some((next, _)) => {
                    return Some(State::new(
                        self.directional_pad_operators.clone(),
                        next.clone(),
                    ))
                }
                None => return None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NumericKeypadPosition {
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    _1,
    _0,
    A,
}

impl NumericKeypadPosition {
    fn neighbors(&self) -> &[(Self, DirectionalKeypadPosition)] {
        match self {
            Self::_9 => &[
                (Self::_9, DirectionalKeypadPosition::A),
                (Self::_8, DirectionalKeypadPosition::Left),
                (Self::_6, DirectionalKeypadPosition::Down),
            ],
            Self::_8 => &[
                (Self::_8, DirectionalKeypadPosition::A),
                (Self::_7, DirectionalKeypadPosition::Left),
                (Self::_9, DirectionalKeypadPosition::Right),
                (Self::_5, DirectionalKeypadPosition::Down),
            ],
            Self::_7 => &[
                (Self::_7, DirectionalKeypadPosition::A),
                (Self::_8, DirectionalKeypadPosition::Right),
                (Self::_4, DirectionalKeypadPosition::Down),
            ],
            Self::_4 => &[
                (Self::_4, DirectionalKeypadPosition::A),
                (Self::_7, DirectionalKeypadPosition::Up),
                (Self::_5, DirectionalKeypadPosition::Right),
                (Self::_1, DirectionalKeypadPosition::Down),
            ],
            Self::_5 => &[
                (Self::_5, DirectionalKeypadPosition::A),
                (Self::_8, DirectionalKeypadPosition::Up),
                (Self::_4, DirectionalKeypadPosition::Left),
                (Self::_6, DirectionalKeypadPosition::Right),
                (Self::_2, DirectionalKeypadPosition::Down),
            ],
            Self::_6 => &[
                (Self::_6, DirectionalKeypadPosition::A),
                (Self::_9, DirectionalKeypadPosition::Up),
                (Self::_5, DirectionalKeypadPosition::Left),
                (Self::_3, DirectionalKeypadPosition::Down),
            ],
            Self::_1 => &[
                (Self::_1, DirectionalKeypadPosition::A),
                (Self::_4, DirectionalKeypadPosition::Up),
                (Self::_2, DirectionalKeypadPosition::Right),
            ],
            Self::_2 => &[
                (Self::_2, DirectionalKeypadPosition::A),
                (Self::_5, DirectionalKeypadPosition::Up),
                (Self::_1, DirectionalKeypadPosition::Left),
                (Self::_3, DirectionalKeypadPosition::Right),
                (Self::_0, DirectionalKeypadPosition::Down),
            ],
            Self::_3 => &[
                (Self::_3, DirectionalKeypadPosition::A),
                (Self::_6, DirectionalKeypadPosition::Up),
                (Self::_2, DirectionalKeypadPosition::Left),
                (Self::A, DirectionalKeypadPosition::Down),
            ],
            Self::_0 => &[
                (Self::_0, DirectionalKeypadPosition::A),
                (Self::_2, DirectionalKeypadPosition::Up),
                (Self::A, DirectionalKeypadPosition::Right),
            ],
            Self::A => &[
                (Self::A, DirectionalKeypadPosition::A),
                (Self::_0, DirectionalKeypadPosition::Left),
                (Self::_3, DirectionalKeypadPosition::Up),
            ],
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            '1' => Self::_1,
            '0' => Self::_0,
            'A' => Self::A,
            _ => panic!("bad numeric pad char {}!", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DirectionalKeypadPosition {
    Left,
    Up,
    Down,
    Right,
    A,
}

impl DirectionalKeypadPosition {
    /// Return all valid next button positions and the
    /// parent operator's input required to reach them
    fn neighbors(&self) -> &[(Self, Self)] {
        match self {
            Self::Up => &[
                (Self::Up, Self::A),
                (Self::A, Self::Right),
                (Self::Down, Self::Down),
            ],
            Self::A => &[
                (Self::A, Self::A),
                (Self::Up, Self::Left),
                (Self::Right, Self::Down),
            ],
            Self::Left => &[(Self::Left, Self::A), (Self::Down, Self::Right)],
            Self::Down => &[
                (Self::Down, Self::A),
                (Self::Left, Self::Left),
                (Self::Up, Self::Up),
                (Self::Right, Self::Right),
            ],
            Self::Right => &[
                (Self::Right, Self::A),
                (Self::Down, Self::Left),
                (Self::A, Self::Up),
            ],
        }
    }
}

impl DijkstraSearchable for Me {
    type Node = State;
    type Cost = usize;

    fn neighbors(&self, previous: &State, previous_cost: usize) -> Vec<(State, usize)> {
        let keys = [
            DirectionalKeypadPosition::Left,
            DirectionalKeypadPosition::Up,
            DirectionalKeypadPosition::Down,
            DirectionalKeypadPosition::Right,
            DirectionalKeypadPosition::A,
        ];
        keys.into_iter()
            .filter_map(|key| {
                previous
                    .execute(key)
                    .map(|state| (state, previous_cost + 1))
            })
            .collect()
    }
}
