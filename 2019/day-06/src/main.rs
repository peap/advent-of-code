use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// With thanks to
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/.

struct System {
    bodies: Vec<Body>,
    map: HashMap<String, usize>,
}

impl System {
    fn new() -> System {
        System { bodies: vec![], map: HashMap::new() }
    }

    fn from_lines(lines: Vec<String>) -> System {
        let mut system = System::new();
        for line in lines.iter() {
            let names: Vec<&str> = line.split(")").collect();
            if names.len() != 2 {
                panic!("Got the wrong number of bodies: {:?}", names);
            }
            let focus_idx = system.add_body(names[0].to_string());
            let body_idx = system.add_body(names[1].to_string());
            system.set_focus(body_idx, focus_idx);
        }
        system
    }

    fn add_body(&mut self, name: String) -> usize {
        let bodies = &mut self.bodies;
        self.map.entry(name.clone()).or_insert_with(|| {
            let next_idx = bodies.len();
            bodies.push(Body {
                focus: None,
                name: name,
            });
            next_idx
        }).clone()
    }

    fn set_focus(&mut self, body_idx: usize, focus_idx: usize) {
        self.bodies[body_idx].focus = Some(focus_idx.clone());
    }

    fn count_orbits(&self, body_idx: usize) -> i32 {
        if let Some(focus_idx) = self.bodies[body_idx].focus {
            1 + self.count_orbits(focus_idx)
        } else {
            0
        }

    }

    fn total_orbits(&self) -> i32 {
        self.map.values().fold(0, |acc, idx| acc + self.count_orbits(*idx))
    }
}

struct Body {
    focus: Option<usize>,
    name: String,
}

fn get_lines(filename: &'static str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    reader.lines().map(|l| l.unwrap().trim().to_string()).collect()
}

fn part1() -> i32 {
    let lines = get_lines("input.txt");
    let system = System::from_lines(lines);
    system.total_orbits()
}

fn main() {
    println!("Part1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = vec![
            String::from("COM)B"),
            String::from("B)C"),
            String::from("C)D"),
            String::from("D)E"),
            String::from("E)F"),
            String::from("B)G"),
            String::from("G)H"),
            String::from("D)I"),
            String::from("E)J"),
            String::from("J)K"),
            String::from("K)L"),
       ];
       let system = System::from_lines(lines);
       assert_eq!(system.map.len(), 12);
       assert_eq!(system.count_orbits(0), 0);
       assert_eq!(system.count_orbits(1), 1);
       assert_eq!(system.count_orbits(2), 2);
       assert_eq!(system.total_orbits(), 42);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 171213);
    }

}
