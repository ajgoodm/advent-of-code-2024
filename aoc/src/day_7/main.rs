use utils::{parse_iter, AocBufReader};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_7/data/part_1.txt"));
}

fn part_1(input: impl Iterator<Item = String>) {
    println!("part 1: {}", part_1_inner(input.map(parse_line)));
}

fn part_1_inner(inputs: impl Iterator<Item = (usize, Vec<usize>)>) -> usize {
    inputs
        .filter(|(test_val, args)| is_solvable(*test_val, args))
        .map(|(test_val, _)| test_val)
        .sum()
}

fn is_solvable(val: usize, args: &[usize]) -> bool {
    let n_args = args.len();

    // this is the base case! If we have two arguments, does their
    // sum or product equal the value?
    if n_args == 2 {
        return args[0] + args[1] == val || args[0] * args[1] == val;
    }

    // because we're multiplying and adding, our accumulated sum / product
    // will only get bigger! If any argument is larger than our final value
    // there's no way to create an equation that yields it.
    if args.iter().any(|x| *x > val) {
        return false;
    }

    let last = *args.last().unwrap();

    // we assume the last operation is addition and recurse
    let is_solvable_by_addition: bool = is_solvable(val - last, &args[..(n_args - 1)]);

    // we assume the last operation is multiplication and recurse
    let is_solvable_by_multiplication: bool =
        { val % last == 0 && is_solvable(val / last, &args[..(n_args - 1)]) };

    is_solvable_by_addition || is_solvable_by_multiplication
}

fn parse_line(line: String) -> (usize, Vec<usize>) {
    let mut test_value_numbers = line.split(":");
    let test_value = test_value_numbers.next().unwrap().parse::<usize>().unwrap();
    (
        test_value,
        parse_iter::<usize, &str>(test_value_numbers.next().unwrap().split_whitespace()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1_inner(
                [
                    "190: 10 19",
                    "3267: 81 40 27",
                    "83: 17 5",
                    "156: 15 6",
                    "7290: 6 8 6 15",
                    "161011: 16 10 13",
                    "192: 17 8 14",
                    "21037: 9 7 18 13",
                    "292: 11 6 16 20",
                ]
                .into_iter()
                .map(|x| parse_line(x.to_string()))
            ),
            3749
        )
    }
}
