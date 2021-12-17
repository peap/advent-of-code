use std::cmp::Ordering;
use std::collections::BinaryHeap;

use common::{default_puzzle, Answer, InputReader, Puzzle};

type Point = (usize, usize);

#[derive(Eq)]
struct Step {
    cost: u64,
    loc: Point,
    dest: Point,
}

impl Step {
    fn new(cost: u64, loc: Point, dest: Point) -> Self {
        Step { cost, loc, dest }
    }

    fn weight(&self) -> usize {
        // let dist = self.x + self.y;
        // dist + dist - self.cost as usize
        (self.dest.0 - self.loc.0) + (self.dest.1 - self.loc.1)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

struct Cave {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Cave {
    fn new(lines: Vec<String>) -> Self {
        let grid: Vec<Vec<u8>> = lines
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let width = grid[0].len();
        let height = grid.len();
        Cave {
            grid,
            width,
            height,
        }
    }

    fn expand5x(&mut self) {
        self.grid.resize(5 * self.height, vec![0; self.width]);
        self.grid
            .iter_mut()
            .for_each(|r| r.resize(5 * self.width, 0));
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.grid[y][x] as usize;
                for j in 0..5 {
                    for i in 0..5 {
                        let xnew = x + i * self.width;
                        let ynew = y + j * self.height;
                        let mut valnew = val + i + j;
                        valnew = if valnew > 9 { valnew % 9 } else { valnew };
                        self.grid[ynew][xnew] = valnew as u8;
                    }
                }
            }
        }
        self.width *= 5;
        self.height *= 5;
    }

    fn get_neighbors(&self, p: Point) -> Vec<(usize, usize)> {
        let x = p.0 as i64;
        let y = p.1 as i64;
        let xmax = self.width as i64;
        let ymax = self.height as i64;
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| (0..xmax).contains(x) && (0..ymax).contains(y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn best_path(&self) -> u64 {
        let xf = self.width - 1;
        let yf = self.height - 1;
        let mut queue = BinaryHeap::new();
        let mut best_costs = vec![vec![u64::MAX; self.height]; self.width];
        best_costs[0][0] = 0;
        for (x, y) in self.get_neighbors((0, 0)).into_iter() {
            let cost = self.grid[y][x] as u64;
            queue.push(Step::new(cost, (x, y), (xf, yf)));
            best_costs[y][x] = cost;
        }
        while !queue.is_empty() {
            let step = queue.pop().unwrap();
            if step.cost > best_costs[yf][xf] {
                continue;
            }
            for (i, j) in self.get_neighbors(step.loc).into_iter() {
                let step_cost = step.cost + self.grid[j][i] as u64;
                if step_cost > best_costs[j][i] {
                    continue;
                }
                if step_cost > best_costs[yf][xf] {
                    continue;
                }
                best_costs[j][i] = step_cost;
                if !(i == xf && j == yf) {
                    queue.push(Step::new(step_cost, (i, j), (xf, yf)));
                }
            }
        }
        best_costs[yf][xf]
    }
}

fn part1(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    let cave = Cave::new(lines);
    cave.best_path()
}

fn part2(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    let mut cave = Cave::new(lines);
    cave.expand5x();
    cave.best_path()
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Chiton");
    puzzle.set_part1(part1, "lowest total risk");
    puzzle.set_part2(part2, "lowest total risk (5x bigger)");
    puzzle
}

fn main() {
    get_puzzle().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let mut cave = Cave::new(lines);
        assert_eq!(cave.best_path(), 40);
        cave.expand5x();
        assert_eq!(cave.best_path(), 315);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(361);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        get_puzzle().test_part2(0);
    }
}
