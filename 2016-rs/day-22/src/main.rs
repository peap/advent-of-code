extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(
        r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T +(\d+)%$"
    ).unwrap();
}

#[derive(Eq, PartialEq)]
pub struct Node {
    x: u32,
    y: u32,
    size_tb: u32,
    used_tb: u32,
    available_tb: u32,
    used_pct: u32,
}

impl Node {
    fn from_line(text: String) -> Node {
        let caps = LINE_RE.captures(&text).expect("No regex match.");
        Node {
            x: caps.at(1).unwrap().parse().unwrap(),
            y: caps.at(2).unwrap().parse().unwrap(),
            size_tb: caps.at(3).unwrap().parse().unwrap(),
            used_tb: caps.at(4).unwrap().parse().unwrap(),
            available_tb: caps.at(5).unwrap().parse().unwrap(),
            used_pct: caps.at(6).unwrap().parse().unwrap(),
        }
    }

    fn can_pair_with(&self, other: &Node) -> bool {
        self != other && self.used_tb > 0 && self.used_tb <= other.available_tb
    }
}

pub fn load_grid<'a>(filename: &'a str) -> Vec<Node> {
    let mut grid = Vec::new();
    let f = File::open(filename).expect("Could not open file.");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    lines.next(); // skip command prompt
    lines.next(); // skip header
    for line in lines {
        match line {
            Ok(text) => grid.push(Node::from_line(text)),
            Err(e) => panic!("Error reading line: {}", e),
        }
    }
    grid
}

pub fn count_viable_pairs(nodes: &Vec<Node>) -> u64 {
    let combos = nodes.iter().combinations(2);
    combos.fold(0u64, |acc, pair| {
        let (a, b) = (pair[0], pair[1]);
        if a.can_pair_with(b) && b.can_pair_with(a) {
            acc + 2
        } else if a.can_pair_with(b) || b.can_pair_with(a) {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    let nodes = load_grid("input.txt");
    let n_pairs = count_viable_pairs(&nodes);
    println!("Part 1: There are {} viable pairs.", n_pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let nodes = load_grid("input.txt");
        let n_pairs = count_viable_pairs(&nodes);
        assert_eq!(n_pairs, 950);
    }
}
