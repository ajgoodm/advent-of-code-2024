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
    computer.execute_program(program)
}

fn part_2(input: AocBufReader) {
    let (_, program) = parse_input(input);
    println!("part 2: {}", part_2_inner(program));
}

fn part_2_inner(_: Program) -> usize {
    0
}

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
    fn execute_program(&mut self, mut program: Program) -> String {
        let mut output: Vec<String> = Vec::new();
        while let Some(out) = self.execute_instruction(&mut program) {
            if !out.is_empty() {
                output.push(out);
            }
        }

        output.join(",")
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
            Instruction::Out => return Some((7 & combo_operand.unwrap()).to_string()),
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
