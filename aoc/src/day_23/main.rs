use std::collections::HashMap;

use itertools::Itertools;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_23/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", part_1_inner(Graph::from_input(input)));
}

fn part_1_inner(graph: Graph) -> usize {
    graph.part_1()
}

struct Graph {
    from_to: HashMap<String, Vec<String>>,
}

impl Graph {
    fn part_1(&self) -> usize {
        self.nodes()
            .into_iter()
            .combinations(3)
            .filter(|threeple| threeple.iter().any(|node| node.starts_with("t")))
            .filter(|threeple| self.is_strongly_connected(threeple))
            .count()
    }

    fn is_strongly_connected(&self, nodes: &[String]) -> bool {
        for first_idx in 0..(nodes.len() - 1) {
            for second_idx in (first_idx + 1)..nodes.len() {
                if !self
                    .neighbors(&nodes[first_idx])
                    .contains(&nodes[second_idx])
                {
                    return false;
                }
            }
        }

        true
    }

    fn neighbors(&self, node: &String) -> Vec<String> {
        self.from_to.get(node).unwrap().clone()
    }

    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut from_to: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let mut f_t = line.split("-");
            let from = f_t.next().unwrap().to_owned();
            let to = f_t.next().unwrap().to_owned();

            from_to.entry(from.clone()).or_default().push(to.clone());
            from_to.entry(to.clone()).or_default().push(from.clone());
        }

        Self { from_to }
    }

    fn nodes(&self) -> Vec<String> {
        self.from_to.keys().cloned().collect()
    }
}
