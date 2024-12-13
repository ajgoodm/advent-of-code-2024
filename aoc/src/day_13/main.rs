use std::sync::LazyLock;

use regex::Regex;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_13/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_13/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input, 0.0));
}

fn part_2(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input, 10000000000000.0));
}

fn part_1_inner(input: AocBufReader, offset: f64) -> usize {
    let equations = LinearEquations::from_input(input, offset);
    equations
        .into_iter()
        .map(|equation_set| {
            solve_linear_system(
                equation_set.x_a,
                equation_set.x_b,
                equation_set.y_a,
                equation_set.y_b,
                equation_set.p_x,
                equation_set.p_y,
            )
        })
        .filter(|(a, b)| *a >= 0.0 && *b >= 0.0 && is_int(*a) && is_int(*b))
        .map(|(a, b)| (a.round() as usize) * 3 + (b.round() as usize))
        .sum()
}

fn is_int(f: f64) -> bool {
    let very_small = 0.000001;
    let diff = (f.round() - f).abs();
    diff < very_small
}

#[derive(Debug)]
struct LinearEquations {
    x_a: f64,
    y_a: f64,
    x_b: f64,
    y_b: f64,
    p_x: f64,
    p_y: f64,
}

static BUTTON_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Button [AB]: X[+-](?<x_coef>[0-9]*), Y[+-](?<y_coef>[0-9]*)").unwrap()
});
static PRIZE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Prize: X=(?<x_coord>\d*), Y=(?<y_coord>\d*)").unwrap());

impl LinearEquations {
    fn from_input(mut reader: AocBufReader, offset: f64) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        while let Some(a_button_str) = reader.next() {
            let (x_a, y_a) = {
                let capture = BUTTON_REGEX.captures(&a_button_str).unwrap();
                (
                    capture["x_coef"].parse::<f64>().unwrap(),
                    capture["y_coef"].parse::<f64>().unwrap(),
                )
            };

            let b_button_str = reader.next().unwrap();
            let (x_b, y_b) = {
                let capture = BUTTON_REGEX.captures(&b_button_str).unwrap();
                (
                    capture["x_coef"].parse::<f64>().unwrap(),
                    capture["y_coef"].parse::<f64>().unwrap(),
                )
            };

            let prize_str = reader.next().unwrap();
            let (p_x, p_y) = {
                let capture = PRIZE_REGEX.captures(&prize_str).unwrap();
                (
                    capture["x_coord"].parse::<f64>().unwrap() + offset,
                    capture["y_coord"].parse::<f64>().unwrap() + offset,
                )
            };

            result.push(Self {
                x_a,
                y_a,
                x_b,
                y_b,
                p_x,
                p_y,
            });
            reader.next();
        }

        result
    }
}

/// Given button parameters Button A: x_a, y_a. Button B: x_b, y_b
/// and prize location p_x, p_y, we're looking for the number of
/// A-presses, a and B-presses, b such that:
///   a * x_a + b * x_b = p_x
///   a * y_a + b * y_b = p_y
///
/// This is a system of 2 linear equations often represented:
///   | x_a  x_b |   | a |   | p_x |
///   |          | * |   | = |     |
///   | y_a  y_b |   | b |   | p_y |
///
/// Either by looking it up or by substituting variables,
/// we can get closed formed solutions below
fn solve_linear_system(x_a: f64, x_b: f64, y_a: f64, y_b: f64, p_x: f64, p_y: f64) -> (f64, f64) {
    let b = (x_a * p_y - y_a * p_x) / (x_a * y_b - x_b * y_a);
    let a = (p_x - x_b * b) / x_a;
    (a, b)
}
