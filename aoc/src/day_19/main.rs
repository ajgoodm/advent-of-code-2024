use std::collections::HashMap;

use rayon::prelude::*;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_19/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_19/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (towels, designs) = parse_input(input);
    println!("part 1: {}", part_1_inner(towels, designs));
}

fn part_1_inner(towels: Vec<String>, designs: Vec<String>) -> usize {
    designs
        .par_iter()
        .filter(|d| {
            let mut cache = HashMap::new();
            let_me_count_the_ways(d, &towels, &mut cache) != 0
        })
        .count()
}

fn part_2(input: AocBufReader) {
    let (towels, designs) = parse_input(input);
    println!("part 2: {}", part_2_inner(towels, designs));
}

fn part_2_inner(towels: Vec<String>, designs: Vec<String>) -> usize {
    designs
        .par_iter()
        .map(|d| {
            let mut cache = HashMap::new();
            let_me_count_the_ways(d, &towels, &mut cache)
        })
        .sum()
}

fn let_me_count_the_ways<'a>(
    design: &'a str,
    towels: &Vec<String>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(cached) = cache.get(design) {
        return *cached;
    }

    let result: usize = towels
        .iter()
        .map(|towel| {
            if design == towel {
                1
            } else if design.ends_with(towel) {
                let remainder: usize = design.len() - towel.len();
                let_me_count_the_ways(&design[..remainder], towels, cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design, result);
    result
}

fn parse_input(mut input: AocBufReader) -> (Vec<String>, Vec<String>) {
    let towels: Vec<String> = input
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();

    input.next(); // blank line

    let mut designs: Vec<String> = vec![];
    for design in input {
        designs.push(design);
    }

    (towels, designs)
}
