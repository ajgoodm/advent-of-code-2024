use std::collections::HashMap;

use utils::{parse_iter, AocBufReader};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_11/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_11/data/part_1.txt"));
}

fn part_1(mut input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input.next().unwrap(), 25));
}

fn part_2(mut input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input.next().unwrap(), 75));
}

fn part_1_inner(input: String, n_blinks: usize) -> usize {
    let mut rock_collection = RockCollection::from_string(input);
    for _ in 0..n_blinks {
        rock_collection.blink()
    }

    rock_collection.count()
}

#[derive(Debug)]
struct RockCollection {
    // a mapping from a rock's number to the number of
    // rocks in the collection with that number
    rock_count: HashMap<usize, usize>,
}

impl RockCollection {
    fn from_string(s: String) -> Self {
        let mut rock_count: HashMap<usize, usize> = HashMap::new();
        for val in parse_iter::<usize, &str>(s.split_whitespace()) {
            *rock_count.entry(val).or_insert(0) += 1;
        }

        Self { rock_count }
    }

    fn blink(&mut self) {
        let mut new_rock_count: HashMap<usize, usize> = HashMap::new();
        for (val, count) in std::mem::take(&mut self.rock_count) {
            match val {
                0 => *new_rock_count.entry(1).or_insert(0) += count,
                _ => {
                    if let Some((a, b)) = maybe_split(val) {
                        *new_rock_count.entry(a).or_insert(0) += count;
                        *new_rock_count.entry(b).or_insert(0) += count;
                    } else {
                        *new_rock_count.entry(val * 2024).or_insert(0) += count;
                    }
                }
            }
        }
        self.rock_count = new_rock_count;
    }

    fn count(&self) -> usize {
        self.rock_count.values().copied().sum()
    }
}

/// If x has an even number of digits, return the
/// two numbers that concatenate to make x. Else return None
fn maybe_split(x: usize) -> Option<(usize, usize)> {
    let s = x.to_string();
    let n_digits = s.len();
    if s.len() % 2 == 0 {
        Some((
            s[..(n_digits / 2)].parse::<usize>().unwrap(),
            s[(n_digits / 2)..].parse::<usize>().unwrap(),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner("125 17".to_string(), 25), 55312);
    }
}
