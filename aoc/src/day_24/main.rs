use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_24/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let mut system = System::from_input(input);
    println!("part 1: {}", system.part_1());
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
enum BinOp {
    XOR(String, String),
    OR(String, String),
    AND(String, String),
}

impl BinOp {
    fn args(&self) -> (String, String) {
        match self {
            BinOp::XOR(a, b) => (a.clone(), b.clone()),
            BinOp::OR(a, b) => (a.clone(), b.clone()),
            BinOp::AND(a, b) => (a.clone(), b.clone()),
        }
    }
}

struct System {
    wires: HashSet<String>,
    wire_states: HashMap<String, bool>,
    dependencies: HashMap<String, BinOp>,
}

impl System {
    fn part_1(&mut self) -> usize {
        let z_wires: Vec<_> = self
            .wires
            .iter()
            .filter(|w| w.starts_with("z"))
            .sorted()
            .cloned()
            .collect();

        z_wires
            .into_iter()
            .enumerate()
            .map(|(idx, wire)| {
                let val = wire[1..].parse::<usize>().unwrap();
                if idx != val {
                    panic!(
                        "Something went wrong - wire {} idx {} doesn't match",
                        wire, idx
                    )
                }

                if self.get_wire_value(&wire) {
                    2usize.pow(idx as u32)
                } else {
                    0
                }
            })
            .sum()
    }

    fn get_wire_value(&mut self, wire: &String) -> bool {
        if let Some(result) = self.wire_states.get(wire) {
            return *result;
        }

        let expression = self.dependencies.get(wire).unwrap().clone();
        let result = match expression {
            BinOp::XOR(a, b) => {
                let a = self.get_wire_value(&a);
                let b = self.get_wire_value(&b);
                a ^ b
            }
            BinOp::OR(a, b) => {
                let a = self.get_wire_value(&a);
                let b = self.get_wire_value(&b);
                a || b
            }
            BinOp::AND(a, b) => {
                let a = self.get_wire_value(&a);
                let b = self.get_wire_value(&b);
                a && b
            }
        };

        self.wire_states.insert(wire.clone(), result);
        result
    }

    fn from_input(mut input: impl Iterator<Item = String>) -> Self {
        let mut wire_states = HashMap::new();
        loop {
            let line = input.next().unwrap();
            if line.is_empty() {
                break;
            }

            let mut wire_val = line.split(": ");
            let wire = wire_val.next().unwrap().to_owned();
            let val = wire_val
                .next()
                .map(|x| match x {
                    "1" => true,
                    "0" => false,
                    _ => panic!("bad bool"),
                })
                .unwrap();
            wire_states.insert(wire, val);
        }

        let mut dependencies = HashMap::new();
        for line in input {
            let mut binop_output = line.split(" -> ");
            let mut binop = binop_output.next().unwrap().split_whitespace();
            let output = binop_output.next().unwrap().to_owned();

            let a = binop.next().unwrap().to_owned();
            let op = binop.next().unwrap();
            let b = binop.next().unwrap().to_owned();

            let binop = match op {
                "XOR" => BinOp::XOR(a, b),
                "AND" => BinOp::AND(a, b),
                "OR" => BinOp::OR(a, b),
                _ => panic!("bad op {}", op),
            };
            dependencies.insert(output, binop);
        }

        let mut wires = HashSet::new();
        wires.extend(wire_states.keys().cloned());
        wires.extend(dependencies.iter().flat_map(|(output, binop)| {
            let (a, b) = binop.args();
            [output.clone(), a, b]
        }));

        Self {
            wires,
            wire_states,
            dependencies,
        }
    }
}
