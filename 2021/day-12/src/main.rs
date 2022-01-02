use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use common::{default_puzzle, Puzzle};

const START: &str = "start";
const END: &str = "end";

#[derive(Clone)]
struct Path {
    rooms: Vec<String>,

    // The special small room that gets two visits.
    special_room: Option<String>,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.rooms == other.rooms
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rooms.hash(state);
    }
}

impl Path {
    fn new(from: String, to: String, special_room: Option<String>) -> Self {
        let rooms = vec![from, to];
        Path {
            rooms,
            special_room,
        }
    }

    fn may_visit_room(&mut self, room: &str) -> bool {
        if room.chars().next().unwrap().is_uppercase() {
            return true;
        }
        let visited = self.rooms.iter().any(|r| r == room);
        if !visited {
            return true;
        }
        let mut allowed = false;
        if let Some(r) = &self.special_room {
            if r == room {
                allowed = true;
                self.special_room = None;
            }
        }
        allowed
    }

    fn maybe_extend(&mut self, room: &str) -> bool {
        let mut extended = true;
        if self.may_visit_room(room) {
            self.rooms.push(room.to_string());
        } else {
            extended = false;
        }
        extended
    }
}

struct CaveSystem {
    connections: HashMap<String, Vec<String>>,
}

impl CaveSystem {
    fn new(lines: &[String]) -> Self {
        let segments: Vec<(String, String)> = lines
            .iter()
            .map(|l| {
                let parts: Vec<&str> = l.split('-').collect();
                assert_eq!(parts.len(), 2);
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect();
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        for (from, to) in segments.iter() {
            if let Some(v) = connections.get_mut(from) {
                v.push(to.to_string());
            } else {
                connections.insert(from.to_string(), vec![to.to_string()]);
            }
            if let Some(v) = connections.get_mut(to) {
                v.push(from.to_string());
            } else {
                connections.insert(to.to_string(), vec![from.to_string()]);
            }
        }
        CaveSystem { connections }
    }

    fn get_small_rooms(&self) -> Vec<String> {
        let mut rooms = vec![];
        for room in self.connections.keys() {
            if room.as_str() == START || room.as_str() == END {
                continue;
            }
            if room.chars().next().unwrap().is_lowercase() {
                rooms.push(room.to_string());
            }
        }
        rooms
    }

    fn count_paths(&self, special_small_room: bool) -> u64 {
        let mut paths: HashSet<Path> = HashSet::new();
        let mut queue = VecDeque::new();
        for dest in self.connections[START].iter() {
            if special_small_room {
                for room in self.get_small_rooms().into_iter() {
                    queue.push_back(Path::new(START.to_string(), dest.to_string(), Some(room)));
                }
            } else {
                queue.push_back(Path::new(START.to_string(), dest.to_string(), None));
            }
        }
        while !queue.is_empty() {
            let path = queue.pop_front().unwrap();
            let last = path.rooms.iter().last().unwrap();
            for dest in self.connections[last].iter() {
                let mut new_path = path.clone();
                if new_path.maybe_extend(dest) {
                    if dest == END {
                        paths.insert(new_path);
                    } else {
                        queue.push_back(new_path);
                    }
                }
            }
        }
        paths.len() as u64
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Passage Pathing");
    puzzle.set_part1("number of paths", |reader| {
        let lines = reader.parsed_lines();
        let cave = CaveSystem::new(&lines);
        cave.count_paths(false)
    });
    puzzle.set_part2("number of special paths", |reader| {
        let lines = reader.parsed_lines();
        let cave = CaveSystem::new(&lines);
        cave.count_paths(true)
    });
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let lines: Vec<String> = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        let cave = CaveSystem::new(&lines);
        assert_eq!(
            HashSet::from_iter(cave.get_small_rooms().iter().map(|r| r.as_str())),
            HashSet::from(["b", "c", "d"])
        );
        assert_eq!(cave.count_paths(false), 10);
        assert_eq!(cave.count_paths(true), 36);
    }

    #[test]
    fn test_example2() {
        let lines: Vec<String> = vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let cave = CaveSystem::new(&lines);
        assert_eq!(
            HashSet::from_iter(cave.get_small_rooms().iter().map(|r| r.as_str())),
            HashSet::from(["dc", "kj", "sa"])
        );
        assert_eq!(cave.count_paths(false), 19);
        assert_eq!(cave.count_paths(true), 103);
    }

    #[test]
    fn test_example3() {
        let lines: Vec<String> = vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let cave = CaveSystem::new(&lines);
        assert_eq!(
            HashSet::from_iter(cave.get_small_rooms().iter().map(|r| r.as_str())),
            HashSet::from(["fs", "he", "pj", "sl", "zg"])
        );
        assert_eq!(cave.count_paths(false), 226);
        assert_eq!(cave.count_paths(true), 3509);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(3576);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(84271);
    }
}
