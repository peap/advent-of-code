use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    ADD,
    MUL,
    INPUT,
    OUTPUT,
    END,
}

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
}

impl From<i32> for ParamMode {
    fn from(i: i32) -> ParamMode {
        match i {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("Unknown ParamMode {}", i),
        }
    }
}

#[derive(Debug)]
struct Opcode {
    instruction: Instruction,
    num_params: usize,
    param_modes: Vec<ParamMode>,
    input: i32,
}

impl From<i32> for Opcode {
    fn from(i: i32) -> Opcode {
        let code = i % 100;
        let (instruction, num_params) = match code {
            1 => (Instruction::ADD, 3),
            2 => (Instruction::MUL, 3),
            3 => (Instruction::INPUT, 1),
            4 => (Instruction::OUTPUT, 1),
            99 => (Instruction::END, 0),
            _ => panic!("Unknown instruction {}", code),
        };
        let mut remainder = i / 100;
        let param_modes = (0..num_params)
            .map(|_| {
                let mode = ParamMode::from(remainder % 10);
                remainder = remainder / 10;
                mode
            })
            .collect();
        Opcode {
            instruction: instruction,
            num_params: num_params,
            param_modes: param_modes,
            input: 0,
        }
    }
}

impl Opcode {
    fn act(&self, pos: &usize, program: &mut Program) -> Option<i32> {
        use Instruction::*;
        let mut params = vec![];
        for n in 0..self.num_params {
            let param_val = program[pos + 1 + n] as usize;
            let value = match self.param_modes[n] {
                ParamMode::Position => program[param_val],
                ParamMode::Immediate => param_val as i32,
            };
            params.push(value);
        }
        let i = if self.num_params >= 1 {
            program[pos + 1] as usize
        } else {
            0
        };
        let k = if self.num_params >= 3 {
            program[pos + 3] as usize
        } else {
            0
        };
        match self.instruction {
            ADD => program[k] = params[0] + params[1],
            MUL => program[k] = params[0] * params[1],
            INPUT => program[i] = self.input,
            OUTPUT => return Some(params[0]),
            END => {}
        };
        None
    }

    fn end(&self) -> bool {
        self.instruction == Instruction::END
    }

    fn set_input(&mut self, input: i32) {
        self.input = input
    }
}

type Program = Vec<i32>;

pub struct Computer {
    program: Program,
    noun: i32,
    verb: i32,
    input: i32,
    outputs: Vec<i32>,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            noun: program[1].clone(),
            verb: program[2].clone(),
            program: program,
            input: 0,
            outputs: vec![],
        }
    }

    pub fn from_file(filename: &'static str) -> Computer {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        Computer::new(
            reader
                .split(b',')
                .map(|c| {
                    String::from_utf8(c.unwrap())
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap()
                })
                .collect(),
        )
    }

    pub fn set_noun_verb(&mut self, noun: i32, verb: i32) {
        self.noun = noun;
        self.verb = verb;
    }

    pub fn set_input(&mut self, input: i32) {
        self.input = input;
    }

    pub fn execute(&mut self) -> i32 {
        let mut program = self.program.clone();
        program[1] = self.noun;
        program[2] = self.verb;
        let mut pos = 0;
        while pos < program.len() {
            let mut opcode = Opcode::from(program[pos]);
            if opcode.end() {
                break;
            }
            opcode.set_input(self.input);
            if let Some(output) = opcode.act(&pos, &mut program) {
                self.outputs.push(output);
            }
            pos += 1 + opcode.num_params
        }
        program[0]
    }

    pub fn final_output(&self) -> Option<&i32> {
        self.outputs.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_examples() {
        let mut comp1 = Computer::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(comp1.execute(), 2);
        let mut comp2 = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(comp2.execute(), 3500);
        let mut comp3 = Computer::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(comp3.execute(), 2);
        let mut comp4 = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(comp4.execute(), 2);
        let mut comp5 = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(comp5.execute(), 30);
    }

    #[test]
    fn test_day05_examples() {
        let mut comp = Computer::new(vec![1002, 4, 3, 4, 33]);
        assert_eq!(comp.execute(), 1002);
    }
}
