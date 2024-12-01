use std::collections::HashMap;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (left, right) = parse_input(input);
    println!("part 1: {}", part_1_inner(left, right));
}

fn part_2(input: AocBufReader) {
    let (left, right) = parse_input(input);
    println!("part 2: {}", part_2_inner(left, right));
}

fn part_1_inner(mut left: Vec<usize>, mut right: Vec<usize>) -> usize {
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| if l >= r { l - r } else { r - l })
        .sum()
}

fn part_2_inner(left: Vec<usize>, right: Vec<usize>) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for val in right.into_iter() {
        *counts.entry(val).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|v| match counts.get(&v) {
            Some(ct) => v * ct,
            None => 0,
        })
        .sum()
}

fn parse_input(reader: AocBufReader) -> (Vec<usize>, Vec<usize>) {
    reader
        .into_iter()
        .map(|s| {
            let mut nums = s.split_whitespace();
            (
                nums.next().unwrap().parse::<usize>().unwrap(),
                nums.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let left: Vec<usize> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<usize> = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(part_1_inner(left, right), 11);
    }

    #[test]
    fn test_part_2() {
        let left: Vec<usize> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<usize> = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(part_2_inner(left, right), 31);
    }
}
