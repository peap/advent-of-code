use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Heading {
    East,
    North,
    West,
    South,
}

#[derive(Debug)]
struct Walker {
    x: i32,  // + east, - west
    y: i32,  // + north, - south
    heading: Heading,
    locations: HashSet<(i32, i32)>,
    bunny_hq: Option<(i32, i32)>,
}

impl Walker {
    fn new() -> Walker {
        Walker {
            x:0,
            y:0,
            heading: Heading::East,
            locations: HashSet::new(),
            bunny_hq: None,
        }
    }

    fn follow(&mut self, instructions: Vec<String>) {
        self.check_for_bunny_hq();
        for instr in instructions {
            self.follow_instruction(&instr);
        }
    }

    fn follow_instruction(&mut self, instruction: &str) {
        let (direction, _distance) = instruction.trim().split_at(1);
        self.heading = self.turn(direction);
        let distance = _distance.parse::<i32>();
        match distance {
            Ok(num) => self.walk(num),
            Err(_) => panic!("Count not parse distance, {}", _distance),
        }
    }

    fn turn(&mut self, direction: &str) -> Heading {
        match direction {
            "L" => {
                match self.heading {
                    Heading::East => Heading::North,
                    Heading::North => Heading::West,
                    Heading::West => Heading::South,
                    Heading::South => Heading::East,
                }
            }
            "R" => {
                match self.heading {
                    Heading::East => Heading::South,
                    Heading::North => Heading::East,
                    Heading::West => Heading::North,
                    Heading::South => Heading::West,
                }
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    fn walk(&mut self, distance: i32) {
        for _ in 0..distance {
            match self.heading {
                Heading::East => self.x += 1,
                Heading::North => self.y += 1,
                Heading::West => self.x -= 1,
                Heading::South => self.y -= 1,
            }
            self.check_for_bunny_hq();
        }
    }

    fn check_for_bunny_hq(&mut self) {
        let location = (self.x, self.y);
        if self.locations.contains(&location) {
            match self.bunny_hq {
                Some(_) => (),
                None => self.bunny_hq = Some(location),
            }
        }
        self.locations.insert(location);
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Walker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Walker<{}, {}>", self.x, self.y)
    }
}


fn parse_instructions(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let instructions: Vec<String> = line.split(",").map(|x| x.to_string()).collect();
    let num = instructions.len();
    println!("Read {} instructions from input.txt", num);
    instructions
}

fn main() {
    let instructions = parse_instructions("input.txt");
    let mut walker = Walker::new();
    walker.follow(instructions);
    println!("{} is {} blocks from the start", walker, &walker.distance());
    match walker.bunny_hq {
        Some((x, y)) => {
            let bunny = x.abs() + y.abs();
            println!("Easter bunny is {} blocks from the start", bunny);
        },
        None => {
            println!("Could not find the easter bunny :(");
        }
    }
}

#[test]
fn easy_walk() {
    let mut walker = Walker::new();
    let instructions: Vec<String> = [
        "R1",
    ].iter().map(|x| x.to_string()).collect();
    walker.follow(instructions);
    assert_eq!(walker.distance(), 1);
    assert_eq!((walker.x, walker.y), (0, -1));
}

#[test]
fn less_easy_walk() {
    let mut walker = Walker::new();
    let instructions: Vec<String> = [
        "R1",
        "R10",
    ].iter().map(|x| x.to_string()).collect();
    walker.follow(instructions);
    assert_eq!(walker.distance(), 11);
    assert_eq!((walker.x, walker.y), (-10, -1));
}

#[test]
fn longer_walker() {
    let mut walker = Walker::new();
    let instructions: Vec<String> = [
        "R1",
        "R10",
        "L50",
    ].iter().map(|x| x.to_string()).collect();
    walker.follow(instructions);
    assert_eq!(walker.distance(), 61);
    assert_eq!((walker.x, walker.y), (-10, -51));
}
