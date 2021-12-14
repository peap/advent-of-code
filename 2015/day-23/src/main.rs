use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use common::{default_puzzle, Answer, BadInput, InputReader, Puzzle};

type Register = char;
type Offset = i32;

lazy_static! {
    static ref HLF_RE: Regex = Regex::new(r"^hlf (a|b)$").unwrap();
    static ref TPL_RE: Regex = Regex::new(r"^tpl (a|b)$").unwrap();
    static ref INC_RE: Regex = Regex::new(r"^inc (a|b)$").unwrap();
    static ref JMP_RE: Regex = Regex::new(r"^jmp ([0-9-+]+)$").unwrap();
    static ref JIE_RE: Regex = Regex::new(r"^jie (a|b), ([0-9-+]+)$").unwrap();
    static ref JIO_RE: Regex = Regex::new(r"^jio (a|b), ([0-9-+]+)$").unwrap();
}

fn parse_register(captures: Option<regex::Match>) -> Register {
    match captures {
        Some(s) => s.as_str().chars().next().unwrap(),
        None => panic!("No register captured."),
    }
}

fn parse_offset(captures: Option<regex::Match>) -> Offset {
    match captures {
        Some(s) => s.as_str().parse::<Offset>().expect("Invalid offset value."),
        None => panic!("No register captured."),
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl FromStr for Instruction {
    type Err = BadInput;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if HLF_RE.is_match(line) {
            let caps = HLF_RE.captures(line).unwrap();
            let register = parse_register(caps.get(1));
            Ok(Instruction::Hlf(register))
        } else if TPL_RE.is_match(line) {
            let caps = TPL_RE.captures(line).unwrap();
            let register = parse_register(caps.get(1));
            Ok(Instruction::Tpl(register))
        } else if INC_RE.is_match(line) {
            let caps = INC_RE.captures(line).unwrap();
            let register = parse_register(caps.get(1));
            Ok(Instruction::Inc(register))
        } else if JMP_RE.is_match(line) {
            let caps = JMP_RE.captures(line).unwrap();
            let offset = parse_offset(caps.get(1));
            Ok(Instruction::Jmp(offset))
        } else if JIE_RE.is_match(line) {
            let caps = JIE_RE.captures(line).unwrap();
            let register = parse_register(caps.get(1));
            let offset = parse_offset(caps.get(2));
            Ok(Instruction::Jie(register, offset))
        } else if JIO_RE.is_match(line) {
            let caps = JIO_RE.captures(line).unwrap();
            let register = parse_register(caps.get(1));
            let offset = parse_offset(caps.get(2));
            Ok(Instruction::Jio(register, offset))
        } else {
            Err(BadInput)
        }
    }
}

pub struct Computer {
    registers: HashMap<Register, i32>,
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        Computer { registers }
    }

    fn get_register(&self, register: char) -> i32 {
        *self.registers.get(&register).unwrap_or(&0)
    }

    fn half(&mut self, register: Register) {
        let current_value = self.get_register(register);
        self.registers.insert(register, current_value / 2);
    }

    fn triple(&mut self, register: Register) {
        let current_value = self.get_register(register);
        self.registers.insert(register, current_value * 3);
    }

    fn increment(&mut self, register: Register) {
        let current_value = self.get_register(register);
        self.registers.insert(register, current_value + 1);
    }

    fn process(&mut self, instructions: &[Instruction]) {
        let mut pos: i32 = 0;
        let max_pos: i32 = instructions.len() as i32 - 1;
        loop {
            let instruction = instructions[pos as usize].clone();
            pos += 1;
            match instruction {
                Instruction::Hlf(reg) => self.half(reg),
                Instruction::Tpl(reg) => self.triple(reg),
                Instruction::Inc(reg) => self.increment(reg),
                Instruction::Jmp(offset) => {
                    pos += offset - 1; // -1 to negate += 1 at top
                }
                Instruction::Jie(reg, offset) => {
                    let reg_val = self.get_register(reg);
                    if reg_val % 2 == 0 {
                        pos += offset - 1; // -1 to negate += 1 at top
                    }
                }
                Instruction::Jio(reg, offset) => {
                    let reg_val = self.get_register(reg);
                    if reg_val == 1 {
                        pos += offset - 1; // -1 to negate += 1 at top
                    }
                }
            }
            if pos < 0 {
                panic!("Was told to jump before the first instruction.");
            }
            if pos > max_pos {
                break;
            }
        }
    }
}

fn part1(reader: &InputReader) -> Answer {
    let instructions = reader.parsed_lines();
    let mut computer = Computer::new();
    computer.process(&instructions);
    computer.get_register('b') as u64
}

fn part2(reader: &InputReader) -> Answer {
    let instructions = reader.parsed_lines();
    let mut computer = Computer::new();
    computer.increment('a');
    computer.process(&instructions);
    computer.get_register('b') as u64
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Opening the Turing Lock");
    puzzle.set_part1(part1, "final value in register b (a=0)");
    puzzle.set_part2(part2, "final value in register b (a=1)");
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(307);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(160);
    }
}
