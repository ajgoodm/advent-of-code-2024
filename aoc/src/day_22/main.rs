use itertools::Itertools;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_22/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_22/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input));
}

fn part_1_inner(input: impl Iterator<Item = String>) -> usize {
    input
        .into_iter()
        .map(|x| hash(x.parse().unwrap(), 2000))
        .sum()
}

fn part_2(input: AocBufReader) {
    println!("part 2: {}", part_2_inner(input));
}

fn part_2_inner(input: impl Iterator<Item = String>) -> usize {
    let mut sequences: Vec<Vec<isize>> = Vec::new();
    let mut val_sequences: Vec<Vec<usize>> = Vec::new();
    for line in input {
        let mut sequence: Vec<isize> = Vec::new();
        let mut val_sequence: Vec<usize> = Vec::new();

        let mut previous = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            let next = next(previous);
            val_sequence.push(next % 10);

            let diff = (next % 10) as isize - (previous % 10) as isize;
            sequence.push(diff);
            previous = next;
        }

        sequences.push(sequence);
        val_sequences.push(val_sequence)
    }

    let mut max: usize = 0;
    for sub_seq in all_sequences() {
        let n_bananas = sequences
            .iter()
            .zip(val_sequences.iter())
            .map(|(haystack, vals)| {
                let needle = (0usize..1997).find(|idx| haystack[*idx..*idx + 4] == sub_seq[..]);

                match needle {
                    Some(idx) => vals[idx + 3],
                    None => 0,
                }
            })
            .sum();

        if n_bananas > max {
            max = n_bananas;
        }
    }

    max
}

fn all_sequences() -> impl Iterator<Item = Vec<isize>> {
    (-9..=9isize)
        .cartesian_product(-9..=9isize)
        .cartesian_product(-9..=9isize)
        .cartesian_product(-9..=9isize)
        .map(|(((a, b), c), d)| vec![a, b, c, d])
}

fn hash(mut input: usize, n: usize) -> usize {
    for _ in 0..n {
        input = next(input);
    }
    input
}

fn next(mut x: usize) -> usize {
    // step one
    x = ((x * 64) ^ x) & 16777215;

    // step two
    x = ((x / 32) ^ x) & 16777215;

    // step three
    x = ((x * 2048) ^ x) & 16777215;

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(next(123), 15887950)
    }
}
