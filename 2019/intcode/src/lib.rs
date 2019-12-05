use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq)]
enum Instruction {
  ADD,
  MUL,
  END,
}

impl From<i32> for Instruction {
    fn from(i: i32) -> Instruction {
        match i {
            1 => Instruction::ADD,
            2 => Instruction::MUL,
            99 => Instruction::END,
            _ => panic!("Unknown instruction"),
        }
    }
}

struct Opcode {
    instruction: Instruction,
    num_params: usize,
}

impl Opcode {
    fn new(i: Instruction) -> Opcode {
        use Instruction::*;
        let num_params = match i {
            ADD => 3,
            MUL => 3,
            END => 0,
        };
        Opcode { instruction: i, num_params: num_params }
    }

    fn act(&self, pos: &usize, program: &mut Program) -> usize {
        use Instruction::*;
        let i = program[pos + 1] as usize;
        let j = program[pos + 2] as usize;
        let k = program[pos + 3] as usize;
        match self.instruction {
          ADD => program[k] = program[i] + program[j],
          MUL => program[k] = program[i] * program[j],
          END => { },
        };
        1 + self.num_params
    }

    fn end(&self) -> bool {
        self.instruction == Instruction::END
    }
}

type Program = Vec<i32>;

pub struct Computer {
    program: Program,
    noun: i32,
    verb: i32,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer {
            noun: program[1].clone(),
            verb: program[2].clone(),
            program: program,
        }
    }

    pub fn from_file(filename: &'static str) -> Computer {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        Computer::new(
            reader.split(b',').map(|c| {
                String::from_utf8(c.unwrap())
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap()
            }).collect()
        )
    }

    pub fn set_noun_verb(&mut self, noun: i32, verb: i32) {
        self.noun = noun;
        self.verb = verb;
    }

    pub fn execute(&self) -> i32 {
        let mut program = self.program.clone();
        program[1] = self.noun;
        program[2] = self.verb;
        let mut pos = 0;
        while pos < program.len() {
            let opcode = Opcode::new(Instruction::from(program[pos]));
            if opcode.end() {
                break;
            }
            pos += opcode.act(&pos, &mut program);
        }
        program[0]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_examples() {
        let comp1 = Computer::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(comp1.execute(), 2);
        let comp2 = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(comp2.execute(), 3500);
        let comp3 = Computer::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(comp3.execute(), 2);
        let comp4 = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(comp4.execute(), 2);
        let comp5 = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(comp5.execute(), 30);
    }

}
