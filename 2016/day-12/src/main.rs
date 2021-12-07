use std::collections::HashMap;

use regex::Regex;

use common::InputReader;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Value {
    Integer(i32),
    Register(char),
}

impl Value {
    fn from_text(text: &str) -> Value {
        let int_re = Regex::new(r"^([0-9-]+)$").expect("Bad integer regex");
        let reg_re = Regex::new(r"^([a-z])$").expect("Bad register regex");
        if int_re.is_match(text) {
            let num: i32 = int_re
                .captures(text)
                .expect("Matched int_re, but no captures???")
                .get(1)
                .expect("Matched int_re, but no match group???")
                .as_str()
                .parse()
                .expect("Matched int_re, but non-numeric???");
            Value::Integer(num)
        } else if reg_re.is_match(text) {
            let reg = reg_re
                .captures(text)
                .expect("Matched reg_re, but no captures???")
                .get(1)
                .expect("Matched reg_re, but no match group???")
                .as_str()
                .chars()
                .next()
                .expect("Matched reg_re, but no first char???");
            Value::Register(reg)
        } else {
            panic!("Got an unparseable Value: {}", text);
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
}

impl From<String> for Instruction {
    fn from(string: String) -> Self {
        let line = &string;
        let cpy_re = Regex::new(r"^cpy (.+) (.+)$").expect("Bad cpy regex.");
        let inc_re = Regex::new(r"^inc (.+)$").expect("Bad inc regex.");
        let dec_re = Regex::new(r"^dec (.+)$").expect("Bad dec regex.");
        let jnz_re = Regex::new(r"^jnz (.+) (.+)$").expect("Bad jnz regex.");
        if cpy_re.is_match(line) {
            let caps = cpy_re.captures(line).unwrap();
            let from_val = Value::from_text(caps.get(1).unwrap().as_str());
            let to_val = Value::from_text(caps.get(2).unwrap().as_str());
            Instruction::Cpy(from_val, to_val)
        } else if inc_re.is_match(line) {
            let caps = inc_re.captures(line).unwrap();
            let val = Value::from_text(caps.get(1).unwrap().as_str());
            Instruction::Inc(val)
        } else if dec_re.is_match(line) {
            let caps = dec_re.captures(line).unwrap();
            let val = Value::from_text(caps.get(1).unwrap().as_str());
            Instruction::Dec(val)
        } else if jnz_re.is_match(line) {
            let caps = jnz_re.captures(line).unwrap();
            let val1 = Value::from_text(caps.get(1).unwrap().as_str());
            let val2 = Value::from_text(caps.get(2).unwrap().as_str());
            Instruction::Jnz(val1, val2)
        } else {
            panic!("Unparsable line: {}", line)
        }
    }
}

struct Computer {
    registers: HashMap<char, i32>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            registers: HashMap::new(),
        }
    }

    fn get_register(&self, register: char) -> i32 {
        *self.registers.get(&register).unwrap_or(&0)
    }

    fn copy(&mut self, from: Value, to: Value) {
        let int_value = match from {
            Value::Integer(num) => num,
            Value::Register(reg) => *self
                .registers
                .get(&reg)
                .expect("Attempted copy from uninitialized register!"),
        };
        let to_reg = match to {
            Value::Integer(_) => panic!("Attempted copy to non-register."),
            Value::Register(reg) => reg,
        };
        self.registers.insert(to_reg, int_value);
    }

    fn increment(&mut self, register: Value) {
        match register {
            Value::Integer(_) => panic!("Attempted to INC a non-register."),
            Value::Register(reg) => {
                if let Some(value) = self.registers.get_mut(&reg) {
                    *value += 1;
                } else {
                    panic!("Asked to increment uninitialized register!");
                }
            }
        }
    }

    fn decrement(&mut self, register: Value) {
        match register {
            Value::Integer(_) => panic!("Attempted to INC a non-register."),
            Value::Register(reg) => {
                if let Some(value) = self.registers.get_mut(&reg) {
                    *value -= 1;
                } else {
                    panic!("Asked to increment uninitialized register!");
                }
            }
        }
    }

    fn process(&mut self, instructions: &[Instruction]) {
        let mut pos: i32 = 0;
        let max_pos: i32 = instructions.len() as i32 - 1;
        loop {
            let instruction = instructions[pos as usize].clone();
            pos += 1;
            match instruction {
                Instruction::Cpy(from, to) => self.copy(from, to),
                Instruction::Inc(reg) => self.increment(reg),
                Instruction::Dec(reg) => self.decrement(reg),
                Instruction::Jnz(val1, val2) => {
                    let first_val = match val1 {
                        Value::Register(reg) => self.get_register(reg),
                        Value::Integer(num) => num,
                    };
                    if first_val != 0 {
                        match val2 {
                            Value::Integer(x) => pos += x - 1,
                            Value::Register(_) => panic!("Bad JNZ instruction."),
                        }
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

fn main() {
    let instructions = InputReader::new("input.txt").converted_lines();
    // Part 1
    let mut computer = Computer::new();
    computer.process(&instructions);
    println!(
        "Part 1: The value in register 'a' is {}.",
        computer.get_register('a')
    );
    // Part 2
    let mut computer2 = Computer::new();
    computer2.copy(Value::Integer(1), Value::Register('c'));
    computer2.process(&instructions);
    println!(
        "Part 2: The value in register 'a' is {}.",
        computer2.get_register('a')
    );
}
