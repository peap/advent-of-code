use std::fs::File;
use std::io::{BufRead, BufReader};

type Program = Vec<i32>;

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

fn load_program(filename: &'static str) -> Program {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
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

fn run_program(program: &Program, noun: i32, verb: i32) -> i32 {
    let mut program = program.clone();
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

fn find_inputs(program: Program, target: i32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&program, noun, verb) == target {
                return (noun, verb);
            }
        }
    }
    panic!("Could not find the target output!");
}

fn main() {
    let program = load_program("input.txt");
    println!("Loaded {} program positions", program.len());

    let output = run_program(&program, 12, 2);
    println!("Part 1: position 0 --> {}", output);

    let (noun, verb) = find_inputs(program, 19690720);
    println!("Part 2: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program_ex1() {
        let program = vec![1, 0, 0, 0, 99];
        assert_eq!(run_program(&program, 0, 0), 2);
    }

    #[test]
    fn test_run_program_ex2() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(run_program(&program, 9, 10), 3500);
    }

    #[test]
    fn test_run_program_ex3() {
        let program = vec![2, 3, 0, 3, 99];
        assert_eq!(run_program(&program, 3, 0), 2);
    }

    #[test]
    fn test_run_program_ex4() {
        let program = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(run_program(&program, 4, 4), 2);
    }

    #[test]
    fn test_run_program_ex5() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(run_program(&program, 1, 1), 30);
    }

    #[test]
    fn test_part1() {
        let program = load_program("input.txt");
        assert_eq!(run_program(&program, 12, 2), 3409710);
    }

    #[test]
    fn test_part2() {
        let program = load_program("input.txt");
        let (noun, verb) = find_inputs(program, 19690720);
        assert_eq!(noun, 79);
        assert_eq!(verb, 12);
    }

}
