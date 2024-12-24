use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_23/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_23/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    println!("part 1: {}", Graph::from_input(input).part_1());
}

fn part_2(input: AocBufReader) {
    println!("part 2: {}", Graph::from_input(input).part_2())
}

struct Graph {
    from_to: HashMap<String, HashSet<String>>,
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

    fn part_2(&self) -> String {
        let maximum_clique = self
            .maximumal_cliques(
                HashSet::new(),
                self.nodes().into_iter().collect(),
                HashSet::new(),
            )
            .into_iter()
            .max_by_key(|maximal_clique| maximal_clique.len())
            .unwrap()
            .into_iter()
            .sorted()
            .collect::<Vec<_>>();

        maximum_clique.join(",")
    }

    fn maximumal_cliques(
        &self,
        r: HashSet<String>,
        mut p: HashSet<String>,
        mut x: HashSet<String>,
    ) -> Vec<HashSet<String>> {
        let mut cliques: Vec<HashSet<String>> = vec![];

        if p.is_empty() && x.is_empty() {
            cliques.push(r);
            return cliques;
        }

        let nodes = p.clone();
        for node in nodes {
            let neighbors = self.neighbors(&node);

            cliques.extend(self.maximumal_cliques(
                HashSet::from([node.clone()]).union(&r).cloned().collect(),
                neighbors.clone().intersection(&p).cloned().collect(),
                neighbors.intersection(&x).cloned().collect(),
            ));

            p.remove(&node);
            x.insert(node);
        }

        cliques
    }

    fn neighbors(&self, node: &String) -> HashSet<String> {
        self.from_to.get(node).unwrap().clone()
    }

    fn from_input(input: impl Iterator<Item = String>) -> Self {
        let mut from_to: HashMap<String, HashSet<String>> = HashMap::new();
        for line in input {
            let mut f_t = line.split("-");
            let from = f_t.next().unwrap().to_owned();
            let to = f_t.next().unwrap().to_owned();

            from_to.entry(from.clone()).or_default().insert(to.clone());
            from_to.entry(to.clone()).or_default().insert(from.clone());
        }

        Self { from_to }
    }

    fn nodes(&self) -> Vec<String> {
        self.from_to.keys().cloned().collect()
    }
}
