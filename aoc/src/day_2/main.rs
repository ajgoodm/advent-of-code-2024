use utils::{parse_iter, AocBufReader};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_2/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input));
}

fn part_1_inner(input: impl Iterator<Item = String>) -> usize {
    input.filter(|x| is_safe(x.to_owned())).count()
}

#[allow(clippy::comparison_chain)]
fn is_safe(s: String) -> bool {
    let mut vals = parse_iter::<usize, &str>(s.split_whitespace());
    let first = vals.next();
    let second = vals.next();
    let is_increasing: bool = match (first, second) {
        (Some(first_), Some(second_)) => {
            if first_ > second_ && first_ - second_ > 3
                || first_ < second_ && second_ - first_ > 3
                || first_ == second_
            {
                // the first and second values are unsafe!
                return false;
            }
            second_ > first_
        }
        _ => {
            // we have fewer than two consecutive values so it must be safe
            return true;
        }
    };

    let mut previous = second.unwrap();
    for val in vals {
        if val > previous {
            if !is_increasing || val - previous > 3 {
                return false;
            }
        } else if val < previous {
            if is_increasing || previous - val > 3 {
                return false;
            }
        } else {
            // two consecutive numbers are equal
            return false;
        }
        previous = val;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
            "2 5 6 7 7",
        ];

        assert_eq!(part_1_inner(input.into_iter().map(|x| x.to_string())), 2);
    }
}
