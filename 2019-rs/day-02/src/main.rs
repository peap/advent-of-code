use std::fs::File;
use std::io::{BufRead, BufReader};

type IntCode = Vec<i32>;

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

fn init_intcode(intcode: &mut IntCode, noun: i32, verb: i32) {
    intcode[1] = noun;
    intcode[2] = verb;
}

fn load_intcode(filename: &'static str) -> IntCode {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.split(b',').map(|c| {
        String::from_utf8(c.unwrap()).unwrap().trim().parse().unwrap()
    }).collect()
}

fn process_intcode(intcode: &mut IntCode) {
    let mut pos = 0;
    while pos < intcode.len() {
        let opcode = intcode[pos];
        if opcode == END {
            break;
        }
        let i = intcode[pos+1] as usize;
        let j = intcode[pos+2] as usize;
        let k = intcode[pos+3] as usize;
        if opcode == ADD {
            intcode[k] = intcode[i] + intcode[j];
        } else if opcode == MUL {
            intcode[k] = intcode[i] * intcode[j];
        } else {
            panic!("IDK what to do with opcode {} at position {}", intcode[pos], pos)
        }
        pos += 4
    }
}

fn find_inputs(intcode: IntCode, target: i32) -> (i32, i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut part2_intcode = intcode.clone();
            init_intcode(&mut part2_intcode, noun, verb);
            process_intcode(&mut part2_intcode);
            if part2_intcode[0] == target {
                return (noun, verb);
            }
        }
    }
    panic!("Could not find the target output!");
}

fn main() {
    let intcode = load_intcode("input.txt");
    println!("Loaded {} intcode positions", intcode.len());

    let mut part1_intcode = intcode.clone();
    init_intcode(&mut part1_intcode, 12, 2);
    process_intcode(&mut part1_intcode);
    println!("Part 1: position 0 --> {}", part1_intcode[0]);

    let (noun, verb) = find_inputs(intcode, 19690720);
    println!("Part 2: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut intcode = load_intcode("input.txt");
        init_intcode(&mut intcode, 12, 2);
        process_intcode(&mut intcode);
        assert_eq!(intcode[0], 3409710);
    }

    #[test]
    fn test_part2() {
        let intcode = load_intcode("input.txt");
        let (noun, verb) = find_inputs(intcode, 19690720);
        assert_eq!(noun, 79);
        assert_eq!(verb, 12);
    }

}
