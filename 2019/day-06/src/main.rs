use common::InputReader;
use std::collections::HashMap;

// With thanks to
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/.

struct System {
    bodies: Vec<Body>,
    map: HashMap<String, usize>,
}

impl System {
    fn new() -> System {
        System {
            bodies: vec![],
            map: HashMap::new(),
        }
    }

    fn from_lines(lines: Vec<String>) -> System {
        let mut system = System::new();
        for line in lines.iter() {
            let names: Vec<&str> = line.split(')').collect();
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
        *self.map.entry(name).or_insert_with(|| {
            let next_idx = bodies.len();
            bodies.push(Body { focus: None });
            next_idx
        })
    }

    fn set_focus(&mut self, body_idx: usize, focus_idx: usize) {
        self.bodies[body_idx].focus = Some(focus_idx);
    }

    fn count_orbits(&self, body_idx: usize) -> i32 {
        if let Some(focus_idx) = self.bodies[body_idx].focus {
            1 + self.count_orbits(focus_idx)
        } else {
            0
        }
    }

    fn total_orbits(&self) -> i32 {
        self.map
            .values()
            .fold(0, |acc, idx| acc + self.count_orbits(*idx))
    }

    fn build_path(&self, idx: usize) -> Vec<usize> {
        let mut path = vec![idx];
        let mut current = idx;
        while let Some(focus_idx) = self.bodies[current].focus {
            path.push(focus_idx);
            current = focus_idx;
        }
        path
    }

    fn minimal_transfer(&self, body_name: &str, target_name: &str) -> i32 {
        let body_idx = self.map.get(body_name).unwrap();
        let target_idx = self.map.get(target_name).unwrap();
        let body_path = self.build_path(*body_idx);
        let target_path = self.build_path(*target_idx);
        let mut body_count = -1;
        for bidx in body_path.iter() {
            let mut target_count = -1;
            for tidx in target_path.iter() {
                if bidx == tidx {
                    return body_count + target_count;
                }
                target_count += 1;
            }
            body_count += 1;
        }
        0
    }
}

struct Body {
    focus: Option<usize>,
}

fn part1() -> i32 {
    let lines = InputReader::new("input.txt").string_lines();
    let system = System::from_lines(lines);
    system.total_orbits()
}

fn part2() -> i32 {
    let lines = InputReader::new("input.txt").string_lines();
    let system = System::from_lines(lines);
    system.minimal_transfer("YOU", "SAN")
}

fn main() {
    println!("Part1: Total orbits: {}", part1());
    println!("Part2: Minimum transfer: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
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
    fn test_example_2() {
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
            String::from("K)YOU"),
            String::from("I)SAN"),
        ];
        let system = System::from_lines(lines);
        assert_eq!(system.build_path(0), vec![0]);
        assert_eq!(system.build_path(1), vec![1, 0]);
        assert_eq!(system.build_path(2), vec![2, 1, 0]);
        assert_eq!(system.minimal_transfer("YOU", "SAN"), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 171213);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 292);
    }
}
