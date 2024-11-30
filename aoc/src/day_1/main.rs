use utils::{parse_iter, AocBufReader};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!(
        "part 1: {}",
        part_1_inner(parse_iter::<usize>(input).collect())
    );
}

fn part_1_inner(_input: Vec<usize>) -> usize {
    0
}
