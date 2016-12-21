#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

lazy_static! {
    static ref SWAP_POSITION_RE: Regex =
        Regex::new("swap position ([0-9]) with position ([0-9])").unwrap();
    static ref SWAP_LETTER_RE: Regex =
        Regex::new("swap letter ([a-z]) with letter ([a-z])").unwrap();
    static ref ROTATE_LEFT_RE: Regex =
        Regex::new("rotate left ([0-9]+) steps?").unwrap();
    static ref ROTATE_RIGHT_RE: Regex =
        Regex::new("rotate right ([0-9]+) steps?").unwrap();
    static ref ROTATE_BY_LETTER_RE: Regex =
        Regex::new("rotate based on position of letter ([a-z])").unwrap();
    static ref REVERSE_RE: Regex =
        Regex::new("reverse positions ([0-9]+) through ([0-9]+)").unwrap();
    static ref MOVE_RE: Regex =
        Regex::new("move position ([0-9]+) to position ([0-9]+)").unwrap();
}

pub enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateByLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn from_text<'a>(text: &'a str) -> Operation {
        use Operation::*;
        if SWAP_POSITION_RE.is_match(text) {
            let caps = SWAP_POSITION_RE.captures(text).unwrap();
            let pos1: usize = caps.at(1).unwrap().parse().unwrap();
            let pos2: usize = caps.at(2).unwrap().parse().unwrap();
            SwapPosition(pos1, pos2)
        } else if SWAP_LETTER_RE.is_match(text) {
            let caps = SWAP_LETTER_RE.captures(text).unwrap();
            let letter1: char = caps.at(1).unwrap().chars().next().unwrap();
            let letter2: char = caps.at(2).unwrap().chars().next().unwrap();
            SwapLetter(letter1, letter2)
        } else if ROTATE_LEFT_RE.is_match(text) {
            let caps = ROTATE_LEFT_RE.captures(text).unwrap();
            let num: usize = caps.at(1).unwrap().parse().unwrap();
            RotateLeft(num)
        } else if ROTATE_RIGHT_RE.is_match(text) {
            let caps = ROTATE_RIGHT_RE.captures(text).unwrap();
            let num: usize = caps.at(1).unwrap().parse().unwrap();
            RotateRight(num)
        } else if ROTATE_BY_LETTER_RE.is_match(text) {
            let caps = ROTATE_BY_LETTER_RE.captures(text).unwrap();
            let letter: char = caps.at(1).unwrap().chars().next().unwrap();
            RotateByLetter(letter)
        } else if REVERSE_RE.is_match(text) {
            let caps = REVERSE_RE.captures(text).unwrap();
            let pos1: usize = caps.at(1).unwrap().parse().unwrap();
            let pos2: usize = caps.at(2).unwrap().parse().unwrap();
            Reverse(pos1, pos2)
        } else if MOVE_RE.is_match(text) {
            let caps = MOVE_RE.captures(text).unwrap();
            let pos1: usize = caps.at(1).unwrap().parse().unwrap();
            let pos2: usize = caps.at(2).unwrap().parse().unwrap();
            Move(pos1, pos2)
        } else {
            panic!("Unrecognized operation: {}", text);
        }
    }

    fn apply_to(&self, chars: &mut Vec<char>) {
        use Operation::*;
        match *self {
            SwapPosition(x, y) => {
                chars.swap(x, y);
            }
            SwapLetter(l1, l2) => {
                for c in chars.iter_mut() {
                    if *c == l1 {
                        *c = l2;
                    } else if *c == l2 {
                        *c = l1;
                    }
                }
            }
            RotateLeft(n) => {
                for _ in 0..n {
                    let removed = chars.remove(0);
                    chars.push(removed);
                }
            }
            RotateRight(n) => {
                for _ in 0..n {
                    let popped = chars.pop().unwrap();
                    chars.insert(0, popped);
                }
            }
            RotateByLetter(letter) => {
                if let Some(idx) = chars.iter().position(|c| *c == letter) {
                    let num: usize = if idx >= 4 { idx + 2 } else { idx + 1 };
                    let op = RotateRight(num);
                    op.apply_to(chars);
                }
            }
            Reverse(x, y) => {
                let mut bottom = x;
                let mut top = y;
                while bottom <= top {
                    chars.swap(bottom, top);
                    bottom += 1;
                    top -= 1;
                }
            }
            Move(x, y) => {
                let removed = chars.remove(x);
                chars.insert(y, removed);
            }
        }
    }
}

pub fn load_operations<'a>(filename: &'a str) -> Vec<Operation> {
    let mut ops = Vec::new();
    let f = File::open(filename).expect("Could not open file.");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(text) => ops.push(Operation::from_text(&text)),
            Err(e) => panic!("Error reading line: {}", e),
        }
    }
    ops
}

pub fn scramble<'a>(operations: &'a Vec<Operation>, input: &'a str) -> String {
    let mut scrambled: Vec<char> = input.chars().collect();
    for op in operations {
        op.apply_to(&mut scrambled);
    }
    scrambled.into_iter().collect()
}

fn main() {
    let operations = load_operations("input.txt");
    let input = "abcdefgh";
    let scrambled = scramble(&operations, input);
    println!("Part 1: {} -> {}", "abcdefgh", scrambled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_step_by_step() {
        use Operation::*;
        let mut code = vec!['a', 'b', 'c', 'd', 'e'];
        SwapPosition(4, 0).apply_to(&mut code);
        assert_eq!(code, vec!['e', 'b', 'c', 'd', 'a']);
        SwapLetter('d', 'b').apply_to(&mut code);
        assert_eq!(code, vec!['e', 'd', 'c', 'b', 'a']);
        Reverse(0, 4).apply_to(&mut code);
        assert_eq!(code, vec!['a', 'b', 'c', 'd', 'e']);
        RotateLeft(1).apply_to(&mut code);
        assert_eq!(code, vec!['b', 'c', 'd', 'e', 'a']);
        Move(1, 4).apply_to(&mut code);
        assert_eq!(code, vec!['b', 'd', 'e', 'a', 'c']);
        Move(3, 0).apply_to(&mut code);
        assert_eq!(code, vec!['a', 'b', 'd', 'e', 'c']);
        RotateByLetter('b').apply_to(&mut code);
        assert_eq!(code, vec!['e', 'c', 'a', 'b', 'd']);
        RotateByLetter('d').apply_to(&mut code);
        assert_eq!(code, vec!['d', 'e', 'c', 'a', 'b']);
    }

    #[test]
    fn test_example() {
        use Operation::*;
        let ops = vec![
            SwapPosition(4, 0),
            SwapLetter('d', 'b'),
            Reverse(0, 4),
            RotateLeft(1),
            Move(1, 4),
            Move(3, 0),
            RotateByLetter('b'),
            RotateByLetter('d'),
        ];
        let scrambled = scramble(&ops, "abcde");
        assert_eq!(scrambled, "decab");
    }

    #[test]
    fn test_part_1() {
        let operations = load_operations("input.txt");
        let input = "abcdefgh";
        let scrambled = scramble(&operations, input);
        assert_eq!(scrambled, "ghfacdbe");
    }

}
