use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction {}", s),
        }
    }
}

struct Path {
    direction: Direction,
    distance: i32,
}

impl Path {
    fn new(code: &str) -> Path {
        let (dir, dist) = code.split_at(1);
        Path {
            direction: Direction::from(dir),
            distance: dist.parse().unwrap(),
        }
    }
}

type Wire = Vec<Path>;

fn load_wires(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn wire_path(wire: &str) -> Wire {
    wire.split(",").map(|c| Path::new(c)).collect()
}

fn main() {
    let raw_strings = load_wires("input.txt");
    let wire_paths: Vec<Wire> = raw_strings.iter().map(|s| wire_path(s)).collect();
    println!("Wire 1 has {} paths", wire_paths[0].len());
    println!("Wire 2 has {} paths", wire_paths[1].len());
}
