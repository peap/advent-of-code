extern crate crypto;

use std::collections::VecDeque;

use crypto::digest::Digest;
use crypto::md5::Md5;

pub const MY_HASH: &'static str = "hhhxzeay";

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Maze {
    passcode: String,
    path: String,
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
}

impl Maze {
    fn new<'a>(passcode: &'a str) -> Maze {
        Maze {
            passcode: passcode.to_string(),
            path: String::new(),
            x: 0,
            y: 0,
            max_x: 3,
            max_y: 3,
        }
    }

    fn is_complete(&self) -> bool {
        self.x == self.max_x && self.y == self.max_y
    }

    fn get_open_doors(&self) -> Vec<Direction> {
        let mut open_doors = Vec::new();
        let mut hasher = Md5::new();
        hasher.input_str(&self.passcode);
        hasher.input_str(&self.path);
        let output = hasher.result_str();
        let mut chars = output.chars();
        for door in [Up, Down, Left, Right].iter() {
            let valid = match door {
                &Direction::Up => self.y > 0,
                &Direction::Down => self.y < self.max_y,
                &Direction::Left => self.x > 0,
                &Direction::Right => self.x < self.max_x,
            };
            let c = chars.next().unwrap();
            if valid && c.is_alphabetic() && c != 'a' {
                open_doors.push(door.clone());
            }
        }
        open_doors
    }

    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.path.push('U');
                self.y -= 1;
            }
            Direction::Down => {
                self.path.push('D');
                self.y += 1;
            }
            Direction::Left => {
                self.path.push('L');
                self.x -= 1;
            }
            Direction::Right => {
                self.path.push('R');
                self.x += 1;
            }
        }
    }

    fn clone_and_move(&self, direction: Direction) -> Maze {
        let mut maze = self.clone();
        maze.go(direction);
        maze
    }
}

pub fn get_shortest_path<'a>(hash: &'a str) -> Option<String> {
    let maze = Maze::new(hash);
    let mut q = VecDeque::new();
    for direction in maze.get_open_doors() {
        q.push_back(maze.clone_and_move(direction));
    }
    while !q.is_empty() {
        let maze = q.pop_front().unwrap();
        if maze.is_complete() {
            return Some(maze.path);
        }
        for direction in maze.get_open_doors() {
            q.push_back(maze.clone_and_move(direction));
        }
    }
    None
}

fn main() {
    let path1 = get_shortest_path(MY_HASH);
    println!("\nPart 1: the path is {:?}", path1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let path = get_shortest_path("hijkl");
        assert_eq!(path, None);
    }

    #[test]
    fn test_example_2() {
        let path = get_shortest_path("ihgpwlah");
        let expected = "DDRRRD".to_string();
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn test_example_3() {
        let path = get_shortest_path("kglvqrro");
        let expected = "DDUDRLRRUDRD".to_string();
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn test_example_4() {
        let path = get_shortest_path("ulqzkmiv");
        let expected = "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string();
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn test_part_1() {
        let path = get_shortest_path(MY_HASH);
        let expected = "DDRUDLRRRD".to_string();
        assert_eq!(path, Some(expected));
    }
}
