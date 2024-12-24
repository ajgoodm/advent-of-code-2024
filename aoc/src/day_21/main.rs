use std::collections::HashMap;
use std::iter;
use std::sync::LazyLock;

use coord_2d::Coord2D;
use itertools::Itertools;

use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_21/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_21/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", solve(input, 2));
}

fn part_2(input: AocBufReader) {
    println!("part 2: {}", solve(input, 25));
}

fn solve(input: impl Iterator<Item = String>, n_operators: usize) -> usize {
    let mut result: usize = 0;
    for line in input {
        let mut cache = HashMap::new();
        let shortest_sequence = shortest_seq_length(line.clone(), 0, n_operators, &mut cache);
        let val = line[..(line.len() - 1)].parse::<usize>().unwrap();
        result += shortest_sequence * val
    }

    result
}

fn shortest_seq_length(
    sequence: String,
    operator_idx: usize,
    n_operators: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(sequence.clone(), operator_idx)) {
        return *cached;
    }

    let keypad = if operator_idx == 0 {
        &NUM_KEYPAD
    } else {
        &DIR_KEYPAD
    };

    // the operator starts at A
    let result = iter::once('A')
        .chain(sequence.chars())
        .tuple_windows::<(_, _)>()
        .map(|(this, next)| {
            let paths = paths(keypad, this, next);
            if operator_idx == n_operators {
                let x = paths.iter().map(|x| x.len()).min();
                if x.is_none() {
                    println!("{} - {} - {:?}", this, next, keypad);
                }
                x.unwrap()
            } else {
                paths
                    .into_iter()
                    .map(|path| shortest_seq_length(path, operator_idx + 1, n_operators, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    cache.insert((sequence, operator_idx), result);
    result
}

fn paths(keypad: &Grid<char>, start: char, end: char) -> Vec<String> {
    let start = keypad.find_one(start);
    let end = keypad.find_one(end);

    let mut result = vec![];
    if let Some(v) = vertical_first_path(keypad, &start, &end) {
        result.push(v);
    }
    if let Some(h) = horizontal_first_path(keypad, &start, &end) {
        result.push(h);
    }

    result
}

fn vertical_first_path(
    keypad: &Grid<char>,
    start: &Coord2D<usize>,
    end: &Coord2D<usize>,
) -> Option<String> {
    let mut path = String::new();
    if start.row < end.row {
        // we're going down
        for r in start.row..end.row {
            if keypad.get(&Coord2D::new(r + 1, start.col)).unwrap() == '*' {
                return None;
            }
            path.push('v')
        }
    } else {
        // we're going up
        for r in end.row..start.row {
            if keypad.get(&Coord2D::new(r, start.col)).unwrap() == '*' {
                return None;
            }
            path.push('^')
        }
    }

    if start.col < end.col {
        // we're going right
        for c in start.col..end.col {
            if keypad.get(&Coord2D::new(end.row, c + 1)).unwrap() == '*' {
                return None;
            }
            path.push('>')
        }
    } else {
        // we're going left
        for c in end.col..start.col {
            if keypad.get(&Coord2D::new(end.row, c)).unwrap() == '*' {
                return None;
            }
            path.push('<')
        }
    }
    path.push('A');
    Some(path)
}

fn horizontal_first_path(
    keypad: &Grid<char>,
    start: &Coord2D<usize>,
    end: &Coord2D<usize>,
) -> Option<String> {
    let mut path = String::new();
    if start.col < end.col {
        // we're going right
        for c in start.col..end.col {
            if keypad.get(&Coord2D::new(start.row, c + 1)).unwrap() == '*' {
                return None;
            }
            path.push('>')
        }
    } else {
        // we're going left
        for c in end.col..start.col {
            if keypad.get(&Coord2D::new(start.row, c)).unwrap() == '*' {
                return None;
            }
            path.push('<')
        }
    }

    if start.row < end.row {
        // we're going down
        for r in start.row..end.row {
            if keypad.get(&Coord2D::new(r + 1, end.col)).unwrap() == '*' {
                return None;
            }
            path.push('v')
        }
    } else {
        // we're going up
        for r in end.row..start.row {
            if keypad.get(&Coord2D::new(r, end.col)).unwrap() == '*' {
                return None;
            }
            path.push('^')
        }
    }
    path.push('A');
    Some(path)
}

static NUM_KEYPAD: LazyLock<Grid<char>> = LazyLock::new(|| {
    Grid::from_line_iter(
        ["789", "456", "123", "*0A"]
            .into_iter()
            .map(|x| x.to_string()),
    )
});

static DIR_KEYPAD: LazyLock<Grid<char>> =
    LazyLock::new(|| Grid::from_line_iter(["*^A", "<v>"].into_iter().map(|x| x.to_string())));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        for path in paths(&DIR_KEYPAD, 'v', '>') {
            println!("{}", path);
        }
    }
}
