use std::collections::HashSet;

use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_19/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (towels, designs) = parse_input(input);
    println!("part 1: {}", part_1_inner(towels, designs));
}

fn part_1_inner(towels: Vec<String>, designs: Vec<String>) -> usize {
    designs
        .into_iter()
        .filter(|d| {
            let design_builder = DesignBuilder::new(d.clone(), &towels);
            shortest_path_length(design_builder, "".to_string(), HashSet::from([d.clone()]))
                .is_some()
        })
        .count()
}

struct DesignBuilder<'d> {
    design: String,
    towels: &'d Vec<String>,
}

impl<'d> DesignBuilder<'d> {
    fn new(design: String, towels: &'d Vec<String>) -> Self {
        DesignBuilder { design, towels }
    }
}

impl<'d> DijkstraSearchable for DesignBuilder<'d> {
    type Node = String;
    type Cost = usize;

    fn neighbors(&self, previous: &String, previous_cost: usize) -> Vec<(String, usize)> {
        self.towels
            .iter()
            .map(|next_towel| {
                let mut next = previous.clone();
                next.push_str(next_towel);
                (next, previous_cost + 1)
            })
            .filter(|(next, _)| next.len() <= self.design.len() && self.design.starts_with(next))
            .collect()
    }
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
