use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Val = i32;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    ADD,
    MUL,
    INPUT,
    OUTPUT,
    JUMPT,
    JUMPF,
    LT,
    EQ,
    RBOFFSET,
    END,
}

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl From<Val> for ParamMode {
    fn from(i: Val) -> ParamMode {
        match i {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unknown ParamMode {}", i),
        }
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
    fn from(i: Val) -> Opcode {
        let code = i % 100;
        let (instruction, num_params) = match code {
            1 => (Instruction::ADD, 3),
            2 => (Instruction::MUL, 3),
            3 => (Instruction::INPUT, 1),
            4 => (Instruction::OUTPUT, 1),
            5 => (Instruction::JUMPT, 2),
            6 => (Instruction::JUMPF, 2),
            7 => (Instruction::LT, 3),
            8 => (Instruction::EQ, 3),
            9 => (Instruction::RBOFFSET, 1),
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
            input: None,
            consumed_input: false,
            needs_input: false,
        }
    }
}

impl Opcode {
    fn act(&mut self, pos: &usize, rb: &usize, program: &mut Program) -> (usize, usize, Option<Val>) {
        use Instruction::*;
        let mut params = vec![];
        for n in 0..self.num_params {
            let param_val = program[pos + 1 + n] as usize;
            let value = match self.param_modes[n] {
                ParamMode::Position => program[param_val],
                ParamMode::Immediate => param_val as Val,
                ParamMode::Relative => program[param_val + rb],
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
        let mut new_pos = pos + 1 + self.num_params;
        let mut new_rb = *rb;
        let mut output = None;
        match self.instruction {
            ADD => program[k] = params[0] + params[1],
            MUL => program[k] = params[0] * params[1],
            INPUT => {
                self.needs_input = true;
                if let Some(input) = self.input {
                    program[i] = input;
                    self.consumed_input = true;
                    self.needs_input = false;
                }
            }
            OUTPUT => {
                output = Some(params[0]);
            }
            JUMPT => {
                if params[0] != 0 {
                    new_pos = params[1] as usize;
                }
            }
            JUMPF => {
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
            RBOFFSET => {
                new_rb += params[0] as usize;
            }
            END => {}
        };
        (new_pos, new_rb, output)
    }

    fn end(&self) -> bool {
        self.instruction == Instruction::END
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
    relative_base: usize,
    inputs: Vec<Val>,
    outputs: Vec<Val>,
    finished: bool,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            program: program,
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
        while self.pos < self.program.len() {
            let mut opcode = Opcode::from(self.program[self.pos]);
            if opcode.end() {
                self.finished = true;
                break;
            }
            if self.inputs.len() > 0 {
                opcode.set_possible_input(self.inputs[0]);
            }
            let (new_pos, rb_delta, output) = opcode.act(&self.pos, &self.relative_base, &mut self.program);
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
            self.relative_base += rb_delta;
        }
        self.program[0]
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
}
