use std::fs::File;
use std::io::{BufRead, BufReader};

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

pub type Program = Vec<i32>;

pub struct Computer {
    program: Program,
}

impl Computer {
    pub fn new(program: Program) -> Computer {
        Computer { program: program }
    }

    pub fn from_file(filename: &'static str) -> Computer {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        Computer { program: reader
            .split(b',')
            .map(|c| {
                String::from_utf8(c.unwrap())
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap()
            })
            .collect()
        }
    }

    pub fn execute(&self, noun: i32, verb: i32) -> i32 {
        let mut program = self.program.clone();
        program[1] = noun;
        program[2] = verb;
        let mut pos = 0;
        while pos < program.len() {
            let opcode = program[pos];
            if opcode == END {
                break;
            }
            let i = program[pos + 1] as usize;
            let j = program[pos + 2] as usize;
            let k = program[pos + 3] as usize;
            if opcode == ADD {
                program[k] = program[i] + program[j];
            } else if opcode == MUL {
                program[k] = program[i] * program[j];
            } else {
                panic!(
                    "IDK what to do with opcode {} at position {}",
                    program[pos], pos
                )
            }
            pos += 4
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
        assert_eq!(comp1.execute(0, 0), 2);
        let comp2 = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(comp2.execute(9, 10), 3500);
        let comp3 = Computer::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(comp3.execute(3, 0), 2);
        let comp4 = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(comp4.execute(4, 4), 2);
        let comp5 = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(comp5.execute(1, 1), 30);
    }

}
