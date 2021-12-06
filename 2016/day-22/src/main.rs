use std::collections::VecDeque;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use common::InputReader;

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T +(\d+)%$").unwrap();
}

pub type Position = (u32, u32);
pub type DataChunk = (Position, u32);

#[derive(Clone, Eq, PartialEq)]
pub struct Node {
    position: Position,
    chunks: Vec<DataChunk>,
    size_tb: u32,
    used_tb: u32,
    available_tb: u32,
    // used_pct: u32,
}

impl Node {
    fn from_line(line: &str) -> Node {
        let caps = LINE_RE.captures(line).expect("No regex match.");
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        let position = (x, y);
        let used = caps.get(4).unwrap().as_str().parse().unwrap();
        let chunk = (position, used);
        Node {
            position,
            chunks: vec![chunk],
            size_tb: caps.get(3).unwrap().as_str().parse().unwrap(),
            used_tb: used,
            available_tb: caps.get(5).unwrap().as_str().parse().unwrap(),
            // used_pct: caps.get(6).unwrap().as_str().parse().unwrap(),
        }
    }

    fn update_storage_stats(&mut self) {
        let tb = self.chunks.iter().fold(0, |acc, c| acc + c.1);
        self.used_tb = tb;
        self.available_tb = self.size_tb - tb;
        // self.used_pct = 100 *
        //    (self.used_tb as f32 / self.available_tb as f32).floor() as u32;
    }

    fn transfer_chunks(&mut self) -> Vec<DataChunk> {
        // only allowed to transfer everything
        let chunks = self.chunks.drain(..).collect();
        self.update_storage_stats();
        chunks
    }

    fn receive_chunks(&mut self, mut chunks: Vec<DataChunk>) {
        self.chunks.append(&mut chunks);
        self.update_storage_stats();
    }

    fn can_pair_with(&self, other: &Node) -> bool {
        self.position != other.position && self.used_tb > 0 && self.used_tb <= other.available_tb
    }

    fn is_adjacent_to(&self, other: &Node) -> bool {
        let (sx, sy) = self.position;
        let (ox, oy) = other.position;
        (sx == ox && (sy as i32 - oy as i32).abs() == 1)
            || (sy == oy && (sx as i32 - ox as i32).abs() == 1)
    }
}

#[derive(Clone)]
pub struct Grid {
    nodes: Vec<Node>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Grid {
        let mut nodes = Vec::new();
        let mut lines = lines.iter();
        lines.next(); // skip command prompt
        lines.next(); // skip header
        for line in lines {
            nodes.push(Node::from_line(line));
        }
        nodes.sort_by(|a, b| a.position.cmp(&b.position));
        Grid { nodes }
    }

    fn print(&self) {
        for node in self.nodes.iter() {
            let (_, y) = node.position;
            if y == 0 {
                println!();
            }
            if node.used_tb > 100 {
                print!("X");
            } else if node.position == (37, 0) {
                print!("G");
            } else if node.used_tb > 0 {
                print!(".");
            } else {
                print!("o");
            }
        }
        println!();
    }

    fn count_viable_pairs(&self) -> u64 {
        let combos = self.nodes.iter().combinations(2);
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

    fn clone_and_move_data(&self, from: Position, to: Position) -> Grid {
        let from_idx = self.nodes.iter().position(|n| n.position == from).unwrap();
        let to_idx = self.nodes.iter().position(|n| n.position == to).unwrap();
        let mut new_grid = self.clone();
        let chunks = {
            let source_node = new_grid.nodes.get_mut(from_idx).unwrap();
            source_node.transfer_chunks()
        };
        {
            let dest_node = new_grid.nodes.get_mut(to_idx).unwrap();
            dest_node.receive_chunks(chunks);
        }
        new_grid
    }

    fn get_possible_states(&self) -> Vec<Grid> {
        let mut valid_transfers = Vec::new();
        for (a, b) in self.nodes.iter().tuple_combinations::<(_, _)>() {
            if a.can_pair_with(b) && a.is_adjacent_to(b) {
                valid_transfers.push((a, b));
            }
            if b.can_pair_with(a) && b.is_adjacent_to(a) {
                valid_transfers.push((b, a));
            }
        }
        let mut states = Vec::new();
        for &(a, b) in valid_transfers.iter() {
            states.push(self.clone_and_move_data(a.position, b.position));
        }
        states
    }

    fn has_target_data_at_origin(&self, chunk: DataChunk) -> bool {
        let origin = self.nodes.iter().find(|n| n.position == (0, 0)).unwrap();
        origin.chunks.contains(&chunk)
    }

    fn optimize_data_movement(&self) -> Option<u32> {
        let target_data = self
            .nodes
            .iter()
            .max_by_key(|n| n.position.0 as i32 - n.position.1 as i32)
            .unwrap()
            .chunks[0];
        let mut q: VecDeque<(Grid, u32)> = VecDeque::new();
        q.push_back((self.clone(), 0));
        while !q.is_empty() {
            print!("\rQueue: {:<}", q.len());
            let (grid, n_steps) = q.pop_front().unwrap();
            if grid.has_target_data_at_origin(target_data) {
                return Some(n_steps);
            }
            for state in grid.get_possible_states() {
                q.push_back((state, n_steps + 1));
            }
        }
        None
    }
}

fn main() {
    let lines = InputReader::new("input.txt").string_lines();
    let grid = Grid::from_lines(lines);
    grid.print();
    println!(
        "Part 1: There are {} viable pairs.",
        grid.count_viable_pairs()
    );
    if let Some(n_steps) = grid.optimize_data_movement() {
        println!("\nPart 2: It takes {} steps to move the data.", n_steps);
    } else {
        println!("\nPart 2: Could not move the data.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_node(position: Position, size_tb: u32) -> Node {
        Node {
            position: position,
            chunks: vec![],
            size_tb: size_tb,
            used_tb: 0,
            available_tb: size_tb,
            // used_pct: 0,
        }
    }

    #[test]
    fn test_part_1() {
        let lines = InputReader::new("input.txt").string_lines();
        let grid = Grid::from_lines(lines);
        let n_pairs = grid.count_viable_pairs();
        assert_eq!(n_pairs, 950);
    }

    #[test]
    fn test_adjacency() {
        let node1 = create_node((1, 1), 1);
        let node2 = create_node((2, 1), 1);
        let node3 = create_node((2, 2), 1);
        // 1 and 2
        assert!(node1.is_adjacent_to(&node2));
        assert!(node2.is_adjacent_to(&node1));
        // 1 and 3
        assert!(!node1.is_adjacent_to(&node3));
        assert!(!node3.is_adjacent_to(&node1));
        // 2 and 3
        assert!(node2.is_adjacent_to(&node3));
        assert!(node3.is_adjacent_to(&node2));
    }

    #[test]
    #[ignore] // Long time
    fn test_example() {
        let lines = InputReader::new("input.txt").string_lines();
        let grid = Grid::from_lines(lines);
        let n_steps = grid.optimize_data_movement();
        assert_eq!(n_steps, Some(7));
    }

    #[test]
    fn test_part_2() {
        // takes 256 steps (17 + 22 + 37 + 5 * 36)
        // done visually with grid.print()
    }
}
