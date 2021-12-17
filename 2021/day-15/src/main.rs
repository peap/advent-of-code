use std::collections::HashMap;

use common::{Answer, InputReader, Puzzle};

// type Point = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
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

    fn get_neighbors(&self, p: Point) -> Vec<Point> {
        let x = p.x as i64;
        let y = p.y as i64;
        let xmax = self.width as i64;
        let ymax = self.height as i64;
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| (0..xmax).contains(x) && (0..ymax).contains(y))
            .map(|(x, y)| Point::from((x as usize, y as usize)))
            .collect()
    }

    fn best_path_cost(&self) -> u64 {
        let start_point = Point::from((0, 0));
        let end_point = Point::from((self.width - 1, self.height - 1));
        let mut best_paths: HashMap<Point, (Point, u64)> = HashMap::new();
        best_paths.insert(start_point, (start_point, 0));
        let mut visited = vec![vec![false; self.height]; self.width];

        let mut current_point = start_point;
        while current_point != end_point {
            visited[current_point.y][current_point.x] = true;
            let destinations = self.get_neighbors(current_point);
            let cost_so_far = best_paths.get(&current_point).unwrap().1;
            for next_point in destinations.into_iter() {
                let next_cost = cost_so_far + self.grid[next_point.y][next_point.x] as u64;
                if let Some(current_best) = best_paths.get_mut(&next_point) {
                    if next_cost < current_best.1 {
                        *current_best = (current_point, next_cost);
                    }
                } else {
                    best_paths.insert(next_point, (current_point, next_cost));
                }
            }
            current_point = *best_paths
                .iter()
                .filter(|(p, _)| !visited[p.y][p.x])
                .min_by(|(_, a), (_, b)| a.1.cmp(&b.1))
                .unwrap()
                .0;
        }
        best_paths.get(&end_point).unwrap().1
    }
}

fn part1(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    let cave = Cave::new(lines);
    cave.best_path_cost()
}

fn part2(reader: &InputReader) -> Answer {
    let lines: Vec<String> = reader.parsed_lines();
    let mut cave = Cave::new(lines);
    cave.expand5x();
    cave.best_path_cost()
}

fn get_puzzle(filename: &'static str) -> Puzzle {
    let mut puzzle = Puzzle::new(2021, 15, "Chiton", filename);
    puzzle.set_part1(part1, "lowest total risk");
    puzzle.set_part2(part2, "lowest total risk (5x bigger)");
    puzzle
}

fn main() {
    get_puzzle("input.txt").run();
    get_puzzle("input2.txt").run();
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
        assert_eq!(cave.best_path_cost(), 40);
        cave.expand5x();
        assert_eq!(cave.best_path_cost(), 315);
    }

    #[test]
    fn test_part1() {
        get_puzzle("input.txt").test_part1(361);
    }

    #[test]
    fn test_another_part1() {
        get_puzzle("input2.txt").test_part1(748);
    }

    #[test]
    #[ignore] // several minutes
    fn test_part2() {
        get_puzzle("input.txt").test_part2(2838);
    }

    #[test]
    #[ignore] // several minutes
    fn test_another_part2() {
        get_puzzle("input2.txt").test_part2(3045);
    }
}
