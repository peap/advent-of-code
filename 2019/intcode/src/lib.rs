extern crate num;
#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Val = i64;

#[derive(Debug, Eq, FromPrimitive, PartialEq)]
enum Instruction {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpT = 5,
    JumpF = 6,
    LT = 7,
    EQ = 8,
    RbOffset = 9,
    End = 99,
}

impl From<Val> for Instruction {
    fn from(value: Val) -> Instruction {
        num::FromPrimitive::from_i64(value)
            .unwrap_or_else(|| panic!("Unknown Instruction {}", value))
    }
}

impl Instruction {
    fn num_params(&self) -> usize {
        use Instruction::*;
        match self {
            Add => 3,
            Mul => 3,
            Input => 1,
            Output => 1,
            JumpT => 2,
            JumpF => 2,
            LT => 3,
            EQ => 3,
            RbOffset => 1,
            End => 0,
        }
    }
}

#[derive(Debug, FromPrimitive)]
enum ParamMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<Val> for ParamMode {
    fn from(value: Val) -> ParamMode {
        num::FromPrimitive::from_i64(value).unwrap_or_else(|| panic!("Unknown ParamMode {}", value))
    }
}

impl ParamMode {
    fn get_modes(code: Val, num: usize) -> Vec<ParamMode> {
        let mut remainder = code / 100;
        (0..num)
            .map(|_| {
                let mode = ParamMode::from(remainder % 10);
                remainder /= 10;
                mode
            })
            .collect()
    }
}

#[derive(Debug)]
struct Opcode {
    instruction: Instruction,
    num_params: usize,
    param_modes: Vec<ParamMode>,
    input: Option<Val>,
    consumed_input: bool,
    needs_input: bool,
}

impl From<Val> for Opcode {
    fn from(code: Val) -> Opcode {
        let instr = Instruction::from(code % 100);
        let num_params = instr.num_params();
        let param_modes = ParamMode::get_modes(code, num_params);
        Opcode {
            instruction: instr,
            num_params,
            param_modes,
            input: None,
            consumed_input: false,
            needs_input: false,
        }
    }
}

impl Opcode {
    fn act(&mut self, pos: &usize, rb: &Val, program: &mut Program) -> (usize, Val, Option<Val>) {
        use Instruction::*;
        let mut idxs = vec![];
        let mut params = vec![];
        for n in 0..self.num_params {
            let param_val = program[pos + 1 + n];
            let (idx, value) = match self.param_modes[n] {
                ParamMode::Position => {
                    let idx = param_val as usize;
                    let value = if param_val < program.len() as Val {
                        program[idx]
                    } else {
                        0
                    };
                    (Some(idx), value)
                }
                ParamMode::Immediate => (None, param_val),
                ParamMode::Relative => {
                    let idx = param_val + *rb;
                    let value = if idx < program.len() as Val {
                        program[idx as usize]
                    } else {
                        0
                    };
                    (Some(idx as usize), value)
                }
            };
            idxs.push(idx);
            params.push(value);
        }
        let k = if self.num_params >= 3 {
            idxs[2].unwrap()
        } else {
            0
        };
        if k >= program.len() {
            program.resize(k + 1, 0);
        }
        let mut new_pos = pos + 1 + self.num_params;
        let mut new_rb = *rb;
        let mut output = None;
        match self.instruction {
            Add => program[k] = params[0] + params[1],
            Mul => program[k] = params[0] * params[1],
            Input => {
                self.needs_input = true;
                if let Some(input) = self.input {
                    program[idxs[0].unwrap()] = input;
                    self.consumed_input = true;
                    self.needs_input = false;
                }
            }
            Output => {
                output = Some(params[0]);
            }
            JumpT => {
                if params[0] != 0 {
                    new_pos = params[1] as usize;
                }
            }
            JumpF => {
                if params[0] == 0 {
                    new_pos = params[1] as usize;
                }
            }
            LT => {
                if params[0] < params[1] {
                    program[k] = 1;
                } else {
                    program[k] = 0;
                }
            }
            EQ => {
                if params[0] == params[1] {
                    program[k] = 1;
                } else {
                    program[k] = 0;
                }
            }
            RbOffset => {
                new_rb += params[0];
            }
            End => {}
        };
        (new_pos, new_rb, output)
    }

    fn end(&self) -> bool {
        self.instruction == Instruction::End
    }

    fn set_possible_input(&mut self, input: Val) {
        self.input = Some(input)
    }
}

type Program = Vec<Val>;

#[derive(Clone)]
pub struct Computer {
    program: Program,
    pos: usize,
    relative_base: Val,
    inputs: Vec<Val>,
    outputs: Vec<Val>,
    finished: bool,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            program,
            pos: 0,
            relative_base: 0,
            inputs: vec![],
            outputs: vec![],
            finished: false,
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

    pub fn set_noun_verb(&mut self, noun: Val, verb: Val) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn set_input(&mut self, input: Val) {
        self.inputs.push(input);
    }

    pub fn execute(&mut self) -> Val {
        loop {
            let mut opcode = Opcode::from(self.program[self.pos]);
            if opcode.end() {
                self.finished = true;
                break;
            }
            if !self.inputs.is_empty() {
                opcode.set_possible_input(self.inputs[0]);
            }
            let (new_pos, new_rb, output) =
                opcode.act(&self.pos, &self.relative_base, &mut self.program);
            if let Some(out) = output {
                self.outputs.push(out);
            }
            if opcode.consumed_input {
                self.inputs.remove(0);
            }
            if opcode.needs_input {
                break;
            }
            self.pos = new_pos;
            self.relative_base = new_rb;
        }
        self.program[0]
    }

    pub fn all_outputs(&self) -> &Vec<Val> {
        &self.outputs
    }

    pub fn final_output(&self) -> Option<&Val> {
        self.outputs.last()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
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
        let mut comp1 = Computer::new(vec![1002, 4, 3, 4, 33]);
        assert_eq!(comp1.execute(), 1002);
        let comp2 = Computer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        // Input below 8 -> 999
        let mut comp2_1 = comp2.clone();
        comp2_1.set_input(7);
        comp2_1.execute();
        assert_eq!(comp2_1.final_output(), Some(&999));
        // Input equal 8 -> 1000
        let mut comp2_2 = comp2.clone();
        comp2_2.set_input(8);
        comp2_2.execute();
        assert_eq!(comp2_2.final_output(), Some(&1000));
        // Input above 8 -> 1001
        let mut comp2_3 = comp2.clone();
        comp2_3.set_input(9);
        comp2_3.execute();
        assert_eq!(comp2_3.final_output(), Some(&1001));
    }

    #[test]
    fn test_day09_examples() {
        // Quine example.
        let input1 = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let expected1 = input1.clone();
        let mut comp1 = Computer::new(input1);
        comp1.execute();
        assert_eq!(comp1.all_outputs(), &expected1);

        // Produces a 16-digit number.
        let mut comp2 = Computer::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        comp2.execute();
        let comp2_out = comp2.final_output().unwrap().clone();
        assert!(comp2_out >= (10 as Val).pow(15) && comp2_out < (10 as Val).pow(16));

        // Produces the large value in the middle.
        let mut comp3 = Computer::new(vec![104, 1125899906842624, 99]);
        comp3.execute();
        assert_eq!(comp3.final_output(), Some(&1125899906842624));
    }
}
