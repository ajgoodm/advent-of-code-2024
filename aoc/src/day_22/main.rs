use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_22/data/part_1.txt"));
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
