use std::collections::VecDeque;

use crypto::digest::Digest;
use crypto::md5::Md5;

pub const MY_HASH: &str = "hhhxzeay";

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
    fn new(passcode: &str) -> Maze {
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
        use Direction::*;
        let mut open_doors = Vec::new();
        let mut hasher = Md5::new();
        hasher.input_str(&self.passcode);
        hasher.input_str(&self.path);
        let output = hasher.result_str();
        let mut chars = output.chars();
        for door in [Up, Down, Left, Right].iter() {
            let valid = match *door {
                Up => self.y > 0,
                Down => self.y < self.max_y,
                Left => self.x > 0,
                Right => self.x < self.max_x,
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

pub fn get_shortest_path(hash: &str) -> Option<String> {
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

pub fn get_longest_path(hash: &str) -> Option<String> {
    let maze = Maze::new(hash);
    let mut longest_path: Option<String> = None;
    let mut q = VecDeque::new();
    for direction in maze.get_open_doors() {
        q.push_back(maze.clone_and_move(direction));
    }
    while !q.is_empty() {
        let maze = q.pop_front().unwrap();
        if maze.is_complete() {
            longest_path = match longest_path {
                Some(path) => {
                    if path.len() < maze.path.len() {
                        Some(maze.path)
                    } else {
                        Some(path)
                    }
                }
                None => Some(maze.path),
            };
            continue;
        }
        for direction in maze.get_open_doors() {
            q.push_back(maze.clone_and_move(direction));
        }
    }
    longest_path
}

fn main() {
    let path1 = get_shortest_path(MY_HASH);
    println!("Part 1: the shortest path is {:?}", path1);
    let path2 = get_longest_path(MY_HASH);
    println!(
        "Part 2: the longest path has {} steps",
        path2.unwrap().len()
    );
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

    #[test]
    fn test_example_2_longest() {
        let path = get_longest_path("ihgpwlah");
        assert_eq!(path.unwrap().len(), 370);
    }

    #[test]
    fn test_example_3_longest() {
        let path = get_longest_path("kglvqrro");
        assert_eq!(path.unwrap().len(), 492);
    }

    #[test]
    fn test_example_4_longest() {
        let path = get_longest_path("ulqzkmiv");
        assert_eq!(path.unwrap().len(), 830);
    }

    #[test]
    fn test_part_2() {
        let path = get_longest_path(MY_HASH);
        assert_eq!(path.unwrap().len(), 398);
    }
}
