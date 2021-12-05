use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

const WATCH_QUEUE_SIZE: bool = false;

type Coordinate = (usize, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Wall,
    Open,
    Waypoint(usize),
}

impl Item {

    fn from_char(chr: char) -> Item {
        match chr {
            '#' => Item::Wall,
            '.' => Item::Open,
            c => {
                if c.is_digit(10) {
                    Item::Waypoint(c.to_digit(10).unwrap() as usize)
                } else {
                    panic!("Unexpected maze Item char: {}", c)
                }
            }
        }
    }

    fn is_waypoint(&self) -> bool {
        match self {
            &Item::Waypoint(_) => true,
            _ => false,
        }
    }

}

#[derive(Clone)]
pub struct Maze {
    items: Vec<Vec<Item>>,
    n_waypoints: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct RobotState {
    coords: Coordinate,
    visited: Vec<bool>,
    n_steps: u32,
}

impl Hash for RobotState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
        self.visited.hash(state);
    }
}

impl RobotState {

    fn new_at_start(coords: Coordinate, n_waypoints: usize) -> RobotState {
        let mut visited = vec![false; n_waypoints];
        visited[0] = true;
        RobotState {
            coords: coords,
            visited: visited,
            n_steps: 0,
        }
    }

    fn has_visited_all(&self) -> bool {
        self.visited.iter().all(|n| *n)
    }

    fn clone_and_move(&self, coords: Coordinate, item: Item) -> RobotState {
        let mut new_state = self.clone();
        new_state.coords = coords;
        match item {
            Item::Waypoint(n) => new_state.visited[n] = true,
            _ => (),
        };
        new_state.n_steps += 1;
        new_state
    }

}

impl Maze {

    fn new(items: Vec<Vec<Item>>) -> Maze {
        let n = items.iter().fold(0, |acc, line| {
            acc + line.iter().filter(|l| l.is_waypoint()).count()
        });
        let max_x = items[0].len() - 1;
        let max_y = items.len() - 1;
        Maze {
            items: items,
            n_waypoints: n,
            max_x: max_x,
            max_y: max_y,
        }
    }

    fn from_file<'a>(filename: &'a str) -> Maze {
        let f = File::open(filename).expect("Could not open file.");
        let reader = BufReader::new(f);
        let mut items = Vec::new();
        for line in reader.lines() {
            let text = line.expect("Could not read line from file.");
            let line_items = text.chars().map(|c| Item::from_char(c));
            items.push(line_items.collect());
        }
        Maze::new(items)
    }

    fn get_waypoint_coords(&self, num: usize) -> Coordinate {
        assert!(num < self.n_waypoints);
        let target = Item::Waypoint(num);
        for (y, line) in self.items.iter().enumerate() {
            for (x, item) in line.iter().enumerate() {
                if *item == target {
                    return (x, y);
                }
            }
        }
        panic!("Could not find {:?}.", target);
    }

    fn get_item_at(&self, coords: Coordinate) -> Item {
        let (x, y) = coords;
        self.items[y][x].clone()
    }

    fn get_valid_steps_from(&self, coords: Coordinate) -> Vec<Coordinate> {
        let mut steps = Vec::new();
        let mut possible_coords = Vec::new();
        let (x, y) = coords;
        if x > 0 {
            possible_coords.push((x - 1, y));
        }
        if x < self.max_x - 1 {
            possible_coords.push((x + 1, y));
        }
        if y > 0 {
            possible_coords.push((x, y - 1));
        }
        if y < self.max_y - 1 {
            possible_coords.push((x, y + 1));
        }
        for new_coords in possible_coords {
            if self.get_item_at(new_coords) != Item::Wall {
                steps.push(new_coords);
            }
        }
        steps
    }

    fn minimize_steps(&self, return_to_start: bool) -> Option<u32> {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        let start_coords = self.get_waypoint_coords(0);
        let start_state = RobotState::new_at_start(start_coords, self.n_waypoints);
        seen.insert(start_state.clone());
        q.push_back(start_state);
        while !q.is_empty() {
            if WATCH_QUEUE_SIZE {
                if q.len() % 1000 < 5 {
                    print!("\rQueue: {}", q.len());
                }
            }
            let state = q.pop_front().unwrap();
            if state.has_visited_all() {
                if WATCH_QUEUE_SIZE {
                    print!("\n");
                }
                if return_to_start && state.coords == start_coords {
                    return Some(state.n_steps);
                }
                if !return_to_start {
                    return Some(state.n_steps);
                }
            }
            for coords in self.get_valid_steps_from(state.coords) {
                let item = self.get_item_at(coords);
                let new_state = state.clone_and_move(coords, item);
                if !seen.contains(&new_state) {
                    seen.insert(new_state.clone());
                    q.push_back(new_state);
                }
            }
        }
        None
    }

}

fn main() {
    let maze = Maze::from_file("input.txt");
    if let Some(n_steps) = maze.minimize_steps(false) {
        println!("\nPart 1: takes {} steps to visit all waypoints", n_steps);
    } else {
        println!("\nPart 1: could not find a path that covers all waypoints");
    }
    if let Some(n_steps) = maze.minimize_steps(true) {
        println!("\nPart 2: takes {} steps to visit all waypoints and return \
                 to start", n_steps);
    } else {
        println!("\nPart 2: could not find a path that covers all waypoints \
                 and return to start");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let maze = Maze::from_file("example.txt");
        let n_steps = maze.minimize_steps(false);
        assert_eq!(n_steps, Some(14));
    }

    #[test]
    #[ignore] // 6743s
    fn test_part_1() {
        let maze = Maze::from_file("input.txt");
        let n_steps = maze.minimize_steps(false);
        assert_eq!(n_steps, Some(498));
    }

    #[test]
    #[ignore] // 6743s
    fn test_part_2() {
        let maze = Maze::from_file("input.txt");
        let n_steps = maze.minimize_steps(true);
        assert_eq!(n_steps, Some(804));
    }

}
