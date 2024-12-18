use itertools::Itertools;

use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_17/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_17/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (computer, program) = parse_input(input);
    println!("part 1: {}", part_1_inner(computer, program));
}

fn part_1_inner(mut computer: Computer, program: Program) -> String {
    let digits = computer.execute_program(program);
    digits
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_2(input: AocBufReader) {
    let (_, program) = parse_input(input);
    println!("part 2: {}", part_2_inner(program));
}

/// The goal of this part is to find a starting value for Register A
/// such that the program outputs itself; this is a quine
///  - https://en.wikipedia.org/wiki/Quine_(computing)
///
/// To help, let's examine our specific program:
///     2,4,1,2,7,5,4,7,1,3,5,5,0,3,3,0
///
/// This has a few steps:
///   1) 2(4) - (Bst, 4) - place last 3 bits from A into B
///   2) 1(2) - (Bxl, 2) - XOR the value in B with (...010)
///   3) 7(5) - (Cdv, 5) - interpret register B as a number, n;
///                        replace register C with all but the
///                        last n bits of register A
///   4) 4(7) - (Bxc, _) - XOR B with C
///   5) 1(3) - (Bxl, 3) - XOR B with (...011)
///   6) 5(5) - (Out, 5) - Output the last 3 bits of register B
///   7) 0(3) - (Adv, 3) - Shift A by 3 bits, removing the 3 least
///                        significant bits.
///   8) 3(0) - (Jnz, 0) - If A is not zero, return to the start
///                        of the program; if A is zero, halt
///
/// From the structure of this program, we know that the length
/// of the intial value of A (in bits) is equal to the length of
/// the program's output (the length of our program * 3 = 48 bits).
/// There are too many possible values (281,474,976,710,656) to
/// brute force, so we need to do something smarter
///
/// Notice that the structure of the program is relatively straightforward
/// Consider the last loop through our program data: we know that register
/// A has exactly 3 bits in it as this point (after this loop it will be
/// empty and the program will halt). During this loop, we place these 3
/// bits in B, place some or all of these 3 bits in C, and then do some
/// XORing. Notably, the outcome (the last digit output by our program)
/// is determined entirely by the 3-bit (octal) number in A!
///
/// We can find all octal numbers that generate the last value of our
/// program (0). There is only 1! We can go backwards one octal digit
/// at a time (appending a new least significant octal digit), run
/// the new candidates through our computer and filter to those that
/// generate the _next_ digit and so on.
fn part_2_inner(program: Program) -> u64 {
    let program_data = program.data.clone();
    let mut stems: Vec<u64> = vec![0];

    for idx in (0..program_data.len()).rev() {
        let to_match = program_data[idx..]
            .iter()
            .map(|x| *x as u64)
            .collect::<Vec<u64>>();

        let mut new_stems: Vec<u64> = Vec::new();
        for (octal_digit, old_stem) in (0..8u64).cartesian_product(stems) {
            let octal_number: u64 = octal_digit + (8 * old_stem);
            let mut computer = Computer::with_register_a(octal_number);
            let output = computer.execute_program(program.clone());

            if output == to_match {
                new_stems.push(octal_number);
            }
        }
        stems = new_stems;
    }

    stems.into_iter().min().unwrap()
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_opcode(x: u8) -> Self {
        match x {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("bad opcode {}", x),
        }
    }
}

#[derive(Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
}

impl Computer {
    fn with_register_a(register_a: u64) -> Self {
        Self {
            register_a,
            register_b: 0,
            register_c: 0,
        }
    }

    fn execute_program(&mut self, mut program: Program) -> Vec<u64> {
        let mut output: Vec<u64> = Vec::new();
        while let Some(out) = self.execute_instruction(&mut program) {
            match out.len() {
                0 => (),
                1 => {
                    let char = out.chars().next().unwrap();
                    output.push(char.to_digit(10u32).unwrap() as u64)
                }
                _ => {
                    panic!("program output invalid octal digit {}", out);
                }
            }
        }

        output
    }

    /// Execute the instruction and return the output; if the output is empty,
    /// return an empty string; if the program tries to access a location not in the data
    /// return None (halt)
    fn execute_instruction(&mut self, program: &mut Program) -> Option<String> {
        let opcode = program.read();
        let operand_code = program.read();

        if opcode.is_none() || operand_code.is_none() {
            return None;
        }

        let instruction = Instruction::from_opcode(opcode.unwrap());
        let combo_operand = self.get_combo_operand(operand_code.unwrap());
        let literal_operand = operand_code.unwrap() as u64;

        match instruction {
            Instruction::Adv => {
                self.register_a >>= combo_operand.unwrap();
            }
            Instruction::Bxl => {
                self.register_b ^= literal_operand;
            }
            Instruction::Bst => {
                self.register_b = 7 & combo_operand.unwrap(); // last 3 bits
            }
            Instruction::Jnz => {
                if self.register_a == 0 {
                    // don't do anything
                } else {
                    program.instruction_pointer = literal_operand as usize;
                }
            }
            Instruction::Bxc => {
                self.register_b ^= self.register_c;
            }
            Instruction::Out => {
                return Some((7 & combo_operand.unwrap()).to_string());
            }
            Instruction::Bdv => self.register_b = self.register_a >> combo_operand.unwrap(),
            Instruction::Cdv => self.register_c = self.register_a >> combo_operand.unwrap(),
        }

        Some("".to_string())
    }

    fn get_combo_operand(&self, operand_code: u8) -> Option<u64> {
        if operand_code == 7 {
            return None;
        }

        let result = match operand_code {
            0 => 0u64,
            1 => 1u64,
            2 => 2u64,
            3 => 3u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("bad operand code {}", operand_code),
        };
        Some(result)
    }
}

#[derive(Debug, Clone)]
struct Program {
    data: Vec<u8>,
    len: usize,
    instruction_pointer: usize,
}

impl Program {
    fn read(&mut self) -> Option<u8> {
        if self.instruction_pointer < self.len {
            let result = self.data[self.instruction_pointer];
            self.instruction_pointer += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn parse_input(mut input: impl Iterator<Item = String>) -> (Computer, Program) {
    let register_a: u64 = input
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let register_b: u64 = input
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let register_c: u64 = input
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    input.next(); // clear blank line
    let data: Vec<u8> = input
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let len = data.len();
    (
        Computer {
            register_a,
            register_b,
            register_c,
        },
        Program {
            data,
            len,
            instruction_pointer: 0,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (computer, program) = parse_input(
            [
                "Register A: 729",
                "Register B: 0",
                "Register C: 0",
                "",
                "Program: 0,1,5,4,3,0",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(
            part_1_inner(computer, program),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn test_part_2() {
        let (_, program) = parse_input(
            [
                "Register A: 729",
                "Register B: 0",
                "Register C: 0",
                "",
                "Program: 0,1,5,4,3,0",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(part_2_inner(program), 117440)
    }
}
