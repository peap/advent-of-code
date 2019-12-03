use std::fs::File;
use std::io::{BufRead, BufReader};

type IntCode = Vec<i32>;

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

fn init_intcode(intcode: &mut IntCode, i: i32, j: i32) {
    intcode[1] = i;
    intcode[2] = j;
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

fn main() {
    let intcode = load_intcode("input.txt");
    println!("Loaded {} intcode positions", intcode.len());
    let mut part1_intcode = intcode.clone();
    init_intcode(&mut part1_intcode, 12, 2);
    process_intcode(&mut part1_intcode);
    println!("Part 1: position 0 --> {}", part1_intcode[0]);
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
}
