use std::collections::{HashSet, VecDeque};

use common::{default_puzzle, Puzzle};

struct HeightMap {
    grid: Vec<Vec<u8>>,
}

impl HeightMap {
    fn from_lines(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = vec![vec![0; width]; height];
        for (y, row) in lines.iter().enumerate() {
            for (x, num) in row.chars().enumerate() {
                grid[y][x] = num.to_digit(10).unwrap() as u8;
            }
        }
        HeightMap { grid }
    }

    fn get_neighbors(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let xmax = (self.grid[0].len()) as i64;
        let ymax = (self.grid.len()) as i64;
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| (0..xmax).contains(x) && (0..ymax).contains(y))
            .collect()
    }

    fn get_height(&self, x: i64, y: i64) -> u64 {
        self.grid[y as usize][x as usize] as u64
    }

    fn get_low_points(&self) -> Vec<(i64, i64)> {
        let mut points = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, h) in row.iter().enumerate() {
                let neigh = self.get_neighbors(x as i64, y as i64);
                if neigh
                    .iter()
                    .all(|(i, j)| self.grid[*j as usize][*i as usize] > *h)
                {
                    points.push((x as i64, y as i64));
                }
            }
        }
        points
    }

    fn get_basin_size(&self, x: i64, y: i64) -> u64 {
        let mut size = 0;
        let mut deq = VecDeque::from([(x, y)]);
        let mut scheduled: HashSet<(i64, i64)> = HashSet::from([(x, y)]);
        while !deq.is_empty() {
            let (x, y) = deq.pop_front().unwrap();
            size += 1;
            let neigh = self.get_neighbors(x, y);
            for (i, j) in neigh.into_iter() {
                if scheduled.contains(&(i, j)) {
                    continue;
                }
                if self.get_height(i, j) < 9 {
                    scheduled.insert((i, j));
                    deq.push_back((i, j));
                }
            }
        }
        size
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Smoke Basin");
    puzzle.set_part1("sum of low-point risk levels", |reader| {
        let lines = reader.parsed_lines();
        let map = HeightMap::from_lines(lines);
        let mut sum = 0;
        for (x, y) in map.get_low_points().into_iter() {
            sum += map.get_height(x, y) + 1;
        }
        sum
    });
    puzzle.set_part2("product of three biggest basins", |reader| {
        let lines = reader.parsed_lines();
        let map = HeightMap::from_lines(lines);
        let mut basins: Vec<u64> = map
            .get_low_points()
            .iter()
            .map(|(x, y)| map.get_basin_size(*x, *y))
            .collect();
        basins.sort_unstable_by(|a, b| b.cmp(a));
        basins.into_iter().take(3).reduce(|acc, b| acc * b).unwrap()
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
    fn test_example() {
        let lines: Vec<String> = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];
        let map = HeightMap::from_lines(lines);
        assert_eq!(map.grid.len(), 5);
        assert_eq!(map.grid[0].len(), 10);
        assert_eq!(map.get_neighbors(0, 0), vec![(1, 0), (0, 1)]);
        assert_eq!(map.get_neighbors(9, 4), vec![(8, 4), (9, 3)]);
        assert_eq!(map.get_low_points().len(), 4);
        // Part 1
        let mut sum = 0;
        for (x, y) in map.get_low_points().into_iter() {
            sum += map.get_height(x, y) as u64 + 1;
        }
        assert_eq!(sum, 15);
        // Part 2
        assert_eq!(map.get_basin_size(1, 0), 3);
        assert_eq!(map.get_basin_size(9, 0), 9);
        assert_eq!(map.get_basin_size(2, 2), 14);
        assert_eq!(map.get_basin_size(6, 4), 9);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(494);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(1048128);
    }
}
