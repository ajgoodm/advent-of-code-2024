use std::sync::LazyLock;

use regex::Regex;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_3/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_3/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input))
}

fn part_2(input: AocBufReader) {
    println!("part 1: {}", part_2_inner(input))
}

fn part_1_inner(input: AocBufReader) -> usize {
    input.into_iter().map(|line| sum_products(&line)).sum()
}

fn part_2_inner(input: AocBufReader) -> usize {
    let input = input.collect::<Vec<String>>().join("");
    gated_sum_products(input)
}

static MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap());

fn sum_products(s: &str) -> usize {
    MUL_REGEX
        .captures_iter(s)
        .map(|cap| cap["left"].parse::<usize>().unwrap() * cap["right"].parse::<usize>().unwrap())
        .sum()
}

fn gated_sum_products(s: String) -> usize {
    let mut do_indices = vec![0usize];
    do_indices.extend(s.match_indices("do()").map(|(idx, _)| idx));
    do_indices.push(s.len());
    do_indices.dedup();
    let mut dont_indices = s.match_indices("don't()").map(|(idx, _)| idx);

    let mut result: usize = 0;
    let mut end: usize = 0;
    for start in do_indices {
        if start < end {
            continue;
        }

        loop {
            match dont_indices.next() {
                Some(end_candidate) => {
                    if end_candidate > start {
                        end = end_candidate;
                        break;
                    }
                }
                None => {
                    end = s.len();
                    break;
                }
            }
        }
        result += sum_products(&s[start..end]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            sum_products("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            gated_sum_products(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ),
            48
        );
    }
}
