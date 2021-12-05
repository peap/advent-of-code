extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    Integer(i32),
    Register(char),
}

impl Value {
    fn from_text<'a>(text: &'a str) -> Value {
        let int_re = Regex::new(r"^([0-9-]+)$").expect("Bad integer regex");
        let reg_re = Regex::new(r"^([a-z])$").expect("Bad register regex");
        if int_re.is_match(text) {
            let num: i32 = int_re
                .captures(text)
                .expect("Matched int_re, but no captures???")
                .at(1)
                .expect("Matched int_re, but no match group???")
                .parse()
                .expect("Matched int_re, but non-numeric???");
            Value::Integer(num)
        } else if reg_re.is_match(text) {
            let reg = reg_re
                .captures(text)
                .expect("Matched reg_re, but no captures???")
                .at(1)
                .expect("Matched reg_re, but no match group???")
                .chars()
                .nth(0)
                .expect("Matched reg_re, but no first char???");
            Value::Register(reg)
        } else {
            panic!("Got an unparseable Value: {}", text);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
}

impl Instruction {
    fn from_line<'a>(line: &'a str) -> Instruction {
        let cpy_re = Regex::new(r"^cpy (.+) (.+)$").expect("Bad cpy regex.");
        let inc_re = Regex::new(r"^inc (.+)$").expect("Bad inc regex.");
        let dec_re = Regex::new(r"^dec (.+)$").expect("Bad dec regex.");
        let jnz_re = Regex::new(r"^jnz (.+) (.+)$").expect("Bad jnz regex.");
        let tgl_re = Regex::new(r"^tgl (.+)$").expect("Bad tgl regex.");
        if cpy_re.is_match(line) {
            let caps = cpy_re.captures(line).unwrap();
            let from_val = Value::from_text(caps.at(1).unwrap());
            let to_val = Value::from_text(caps.at(2).unwrap());
            Instruction::Cpy(from_val, to_val)
        } else if inc_re.is_match(line) {
            let caps = inc_re.captures(line).unwrap();
            let val = Value::from_text(caps.at(1).unwrap());
            Instruction::Inc(val)
        } else if dec_re.is_match(line) {
            let caps = dec_re.captures(line).unwrap();
            let val = Value::from_text(caps.at(1).unwrap());
            Instruction::Dec(val)
        } else if jnz_re.is_match(line) {
            let caps = jnz_re.captures(line).unwrap();
            let val1 = Value::from_text(caps.at(1).unwrap());
            let val2 = Value::from_text(caps.at(2).unwrap());
            Instruction::Jnz(val1, val2)
        } else if tgl_re.is_match(line) {
            let caps = tgl_re.captures(line).unwrap();
            let val = Value::from_text(caps.at(1).unwrap());
            Instruction::Tgl(val)
        } else {
            panic!("Unparsable line: {}", line)
        }
    }
}

pub struct Computer {
    registers: HashMap<char, i32>,
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);
        Computer {
            registers: registers,
        }
    }

    fn get_num(&self, value: Value) -> i32 {
        match value {
            Value::Integer(num) => num,
            Value::Register(reg) => self.get_register(reg),
        }
    }

    fn get_register(&self, register: char) -> i32 {
        self.registers.get(&register).unwrap_or(&0).clone()
    }

    fn copy(&mut self, from: Value, to: Value) {
        let int_value = self.get_num(from);
        match to {
            Value::Integer(_) => panic!("Can't copy to a non-register."),
            Value::Register(reg) => {
                self.registers.insert(reg, int_value);
            }
        }
    }

    fn increment(&mut self, register: Value) {
        match register {
            Value::Integer(_) => panic!("Attempted to INC a non-register."),
            Value::Register(reg) => {
                if let Some(value) = self.registers.get_mut(&reg) {
                    *value += 1;
                }
            }
        }
    }

    fn decrement(&mut self, register: Value) {
        match register {
            Value::Integer(_) => panic!("Attempted to DEC a non-register."),
            Value::Register(reg) => {
                if let Some(value) = self.registers.get_mut(&reg) {
                    *value -= 1;
                }
            }
        }
    }

    fn toggle(&self, instruction: &Instruction) -> Instruction {
        use Instruction::*;
        match *instruction {
            Cpy(val1, val2) => Jnz(val1, val2),
            Jnz(val1, val2) => Cpy(val1, val2),
            Inc(val) => Dec(val),
            Dec(val) => Inc(val),
            Tgl(val) => Inc(val),
        }
    }

    fn process(&mut self, instructions: &Vec<Instruction>) {
        let mut instructions = instructions.clone();
        let mut pos: i32 = 0;
        let max_pos: i32 = instructions.len() as i32 - 1;
        loop {
            let instruction = instructions.get(pos as usize).unwrap().clone();
            // println!(
            //     "pos: {:>2}, a: {:>6}, b: {:>6}, c: {:>6}, d: {:>6} - {:?}",
            //     pos + 1, self.get_register('a'), self.get_register('b'),
            //     self.get_register('c'), self.get_register('d'), instruction
            // );
            match instruction {
                Instruction::Cpy(from, to) => self.copy(from, to),
                Instruction::Inc(reg) => self.increment(reg),
                Instruction::Dec(reg) => self.decrement(reg),
                Instruction::Jnz(val1, val2) => {
                    if self.get_num(val1) != 0 {
                        pos += self.get_num(val2) - 1;
                    }
                }
                Instruction::Tgl(val) => {
                    let num = self.get_num(val);
                    let tgl_pos = pos as usize + num as usize;
                    if tgl_pos <= max_pos as usize {
                        let old_instr = instructions.remove(tgl_pos);
                        if tgl_pos < instructions.len() {
                            instructions.insert(tgl_pos, self.toggle(&old_instr));
                        } else if tgl_pos == instructions.len() {
                            instructions.push(self.toggle(&old_instr));
                        }
                    }
                }
            }
            pos += 1;
            if pos > max_pos {
                break;
            }
        }
    }
}

pub fn load_instructions<'a>(filename: &'a str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let f = File::open(filename).expect("Couldn't open file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        if let Ok(text) = line {
            instructions.push(Instruction::from_line(&text));
        }
    }
    instructions
}

fn main() {
    let instructions = load_instructions("input.txt");
    // Part 1
    let mut computer = Computer::new();
    computer.copy(Value::Integer(7), Value::Register('a'));
    computer.process(&instructions);
    println!(
        "Part 1: The value in register 'a' is {}.",
        computer.get_register('a')
    );
    // Part 2
    let mut computer2 = Computer::new();
    computer2.copy(Value::Integer(12), Value::Register('a'));
    computer2.process(&instructions);
    println!(
        "Part 2: The value in register 'a' is {}.",
        computer2.get_register('a')
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        use Instruction::*;
        use Value::*;
        let instructions = vec![
            Cpy(Integer(2), Register('a')),
            Tgl(Register('a')),
            Tgl(Register('a')),
            Tgl(Register('a')),
            Cpy(Integer(1), Register('a')),
            Dec(Register('a')),
            Dec(Register('a')),
        ];
        let mut computer = Computer::new();
        computer.process(&instructions);
        assert_eq!(computer.get_register('a'), 3);
    }

    #[test]
    fn test_part_1() {
        let instructions = load_instructions("input.txt");
        let mut computer = Computer::new();
        computer.copy(Value::Integer(7), Value::Register('a'));
        computer.process(&instructions);
        assert_eq!(computer.get_register('a'), 11340);
    }

    #[test]
    #[ignore] // 2619s
    fn test_part_2() {
        let instructions = load_instructions("input.txt");
        let mut computer = Computer::new();
        computer.copy(Value::Integer(12), Value::Register('a'));
        computer.process(&instructions);
        assert_eq!(computer.get_register('a'), 479007900);
    }
}
