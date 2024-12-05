use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::{FromStr, Split};

use thiserror::Error;

use utils::{parse_iter, AocBufReader};

fn main() {
    order_pages(AocBufReader::from_string("aoc/src/day_5/data/test.txt"));
}

fn order_pages(mut input: impl Iterator<Item = String>) {
    let mut rules: Vec<Ordering> = vec![];
    while let Ok(rule) = input.next().unwrap().parse::<Ordering>() {
        rules.push(rule);
    }

    let mut part_1: usize = 0;
    let mut part_2: usize = 0;

    for line in input {
        match sort_invalid(line, &rules) {
            SortStatus::Sorted(x) => {
                part_1 += x;
            }
            SortStatus::NotSorted(x) => part_2 += x,
        }
    }

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

fn sort_invalid(line: String, rules: &Vec<Ordering>) -> SortStatus {
    let nums: Vec<usize> = parse_iter::<usize, &str>(line.split(",")).collect();
    let middle = nums[nums.len() / 2];

    let val_to_position: HashMap<usize, usize> = nums
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx))
        .collect();

    let mut is_sorted = true;
    for rule in rules {
        if let (Some(first), Some(later)) = (
            val_to_position.get(&rule.first),
            val_to_position.get(&rule.later),
        ) {
            if later < first {
                is_sorted = false;
                break;
            }
        }
    }

    if is_sorted {
        SortStatus::Sorted(middle)
    } else {
        SortStatus::NotSorted(sort(nums, rules))
    }
}

fn sort(_vals: Vec<usize>, _rules: &[Ordering]) -> usize {
    1
}

enum SortStatus {
    Sorted(usize),
    NotSorted(usize),
}

#[derive(Debug)]
struct Ordering {
    first: usize,
    later: usize,
}

impl Ordering {
    fn new(first: usize, later: usize) -> Self {
        Self { first, later }
    }
}

#[derive(Error, Debug)]
enum ParseOrderError {
    #[error("failed to parse int")]
    IntParseFailed(#[from] ParseIntError),
    #[error("split iterator failed")]
    BadSplit,
}

fn _parse(iter: &mut Split<'_, &str>) -> Result<usize, ParseOrderError> {
    let val = match iter.next() {
        Some(x) => x,
        None => return Err(ParseOrderError::BadSplit),
    };

    Ok(val.parse::<usize>()?)
}

impl FromStr for Ordering {
    type Err = ParseOrderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first_later = s.split("|");
        let first: usize = _parse(&mut first_later)?;
        let later: usize = _parse(&mut first_later)?;
        Ok(Self::new(first, later))
    }
}
