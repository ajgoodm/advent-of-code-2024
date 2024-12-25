use std::collections::{HashMap, HashSet};
use std::iter;

use itertools::Itertools;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_24/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_24/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let mut adding_machine = AddingMachine::from_input(input);
    println!("part 1: {}", adding_machine.part_1());
}
/// The adding machine is a standard design
/// https://content.instructables.com/F3M/5KQ6/GHZ6XYFE/F3M5KQ6GHZ6XYFE.bmp
///
/// the value of an output bit z_n is the XOR of the two input bits
/// (x_n and y_n) further XORed with a value that represents
/// "carrying the 1" when the output of the previous bit had 3
/// one values (x_n_1, y_n_1 were both one or the inputs to the
/// prebious bit were both 1). We can debug the buggy adding machine
/// by printing out the sum of incrementally larger numbers composed
/// entirely of 1's and observing when the sum has an error
///
///              z_n
///               |
///               ^
///             /XOR\
///            <----->
///           /       \
///   (in_n_a) A         B (in_n_b)
///          |          \__
///          ^             ^
///        /OR \         /XOR\
///       <----->       <----->
///       |     |       |     |
///       C     D       x_n   y_n
///       |      \____
///       ^           ^
///     /AND\       /AND\
///    <----->     <----->__
///    |     |     |        \
///  x_n-1 y_n-1  in_n-1_a  in_n-1_b
///
///
fn part_2(input: AocBufReader) {
    let original_adding_machine = AddingMachine::from_input(input);

    // all of the initial wire states (constant inputs) are arguments to
    // our adding machine, x\d{2} or y\d{2}.
    // our inputs have 45 bits
    for wire in original_adding_machine.wire_states.keys() {
        assert!(wire.starts_with('x') || wire.starts_with('y'))
    }

    let fixes = [
        ("vkq", "z11"),
        ("z24", "mmk"),
        ("pvb", "qdq"),
        ("hqh", "z38"),
    ];

    let mut adding_machine_v1 = original_adding_machine.clone();
    for &(x, y) in fixes.iter() {
        adding_machine_v1.swap(x, y);
    }

    // prior to fixing the adding machine, this block prints output like this,
    // revealing some errors. This is helpful because adding is recursive with
    // each bit relying on the state of less significant bits.
    //
    //   ** arguments have 11 ones! vkq <-> z11
    //   z: 01111 11111 10100 000000000000000000000000000000
    //
    //   ** argumenst have 24 ones! z24 <-> mmk
    //   z: 01111 11111 11111 11111 11110 10000000000000000000
    //
    //   ** arguments have 29 ones! pvp <-> qdq
    //   z: 01111 11111 11111 11111 11111 11101 000000000000000
    //
    //   ** arguments have 38 ones! z38 <-> hqh
    //   z: 01111 11111 11111 11111 11111 11111 11111 11101 00000
    for n_ones in 0..45 {
        let all_ones = iter::repeat_n('1', n_ones).collect::<String>();
        let mut adding_machine = adding_machine_v1.clone();
        adding_machine.set_inputs(&all_ones, &all_ones);
        println!("\n** arguments have {} ones!", n_ones);
        adding_machine.print_z();
    }

    let sorted_wires: Vec<_> = fixes
        .into_iter()
        .flat_map(|(x, y)| iter::once(x).chain(iter::once(y)))
        .sorted()
        .collect();
    println!("part 2: {}", sorted_wires.join(","));
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

#[derive(Clone)]
struct AddingMachine {
    wires: HashSet<String>,
    wire_states: HashMap<String, bool>,
    dependencies: HashMap<String, BinOp>,
}

impl AddingMachine {
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

    fn swap(&mut self, output_1: &str, output_2: &str) {
        let x = self.dependencies.get(output_1).unwrap().clone();
        let y = self.dependencies.get(output_2).unwrap().clone();

        self.dependencies.insert(output_1.to_string(), y);
        self.dependencies.insert(output_2.to_string(), x);
    }

    fn print_z(&mut self) {
        println!(
            "z: {}",
            (0..45)
                .map(|idx| {
                    match self.get_wire_value(&AddingMachine::z_key_for_idx(idx)) {
                        true => '1',
                        false => '0',
                    }
                })
                .collect::<String>()
        );
    }

    /// take values for x in y in binary represented as strings
    /// of 0 and 1; zero out x and y and set the relevant bits
    /// (starting at least significant)
    fn set_inputs(&mut self, x: &str, y: &str) {
        if x.len() > 45 || y.len() > 45 {
            panic!("adding machine only has 45 bits")
        }

        for idx in 0..45 {
            self.wire_states
                .insert(AddingMachine::x_key_for_idx(idx), false);
            self.wire_states
                .insert(AddingMachine::y_key_for_idx(idx), false);
        }

        for (idx, c) in x.chars().enumerate() {
            let val = match c {
                '0' => false,
                '1' => true,
                _ => panic!("there's no such thing as 3s"),
            };
            self.wire_states
                .insert(AddingMachine::x_key_for_idx(idx), val);
        }

        for (idx, c) in y.chars().enumerate() {
            let val = match c {
                '0' => false,
                '1' => true,
                _ => panic!("there's no such thing as 3s"),
            };
            self.wire_states
                .insert(AddingMachine::y_key_for_idx(idx), val);
        }
    }

    fn x_key_for_idx(idx: usize) -> String {
        format!("x{:0>2}", idx)
    }

    fn y_key_for_idx(idx: usize) -> String {
        format!("y{:0>2}", idx)
    }

    fn z_key_for_idx(idx: usize) -> String {
        format!("z{:0>2}", idx)
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
