use common::{default_puzzle, Puzzle};

struct DumboOctopi {
    grid: Vec<Vec<u8>>,
    flashes: u64,
    steps: u64,
}

impl DumboOctopi {
    fn new(lines: Vec<String>) -> Self {
        let mut grid = vec![vec![0; 10]; 10];
        for (y, line) in lines.iter().enumerate() {
            for (x, lvl) in line.chars().enumerate() {
                grid[y][x] = lvl.to_digit(10).unwrap() as u8;
            }
        }
        DumboOctopi {
            grid,
            flashes: 0,
            steps: 0,
        }
    }

    fn get_neighbors(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let xmax = (self.grid[0].len()) as i64;
        let ymax = (self.grid.len()) as i64;
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .into_iter()
        .filter(|(x, y)| (0..xmax).contains(x) && (0..ymax).contains(y))
        .collect()
    }

    fn step(&mut self) -> u64 {
        self.steps += 1;
        let mut done = false;
        let mut flashed = vec![vec![false; 10]; 10];
        // Add 1 to all points.
        for row in self.grid.iter_mut() {
            for level in row.iter_mut() {
                *level += 1;
            }
        }
        // Resolve flashes.
        while !done {
            done = true;
            let mut new_flashes = vec![vec![false; 10]; 10];
            for (y, row) in self.grid.iter().enumerate() {
                for (x, level) in row.iter().enumerate() {
                    if *level > 9 && !flashed[y][x] {
                        done = false;
                        self.flashes += 1;
                        flashed[y][x] = true;
                        new_flashes[y][x] = true;
                    }
                }
            }
            for (y, row) in new_flashes.iter().enumerate() {
                for (x, f) in row.iter().enumerate() {
                    if *f {
                        for (i, j) in self.get_neighbors(x as i64, y as i64).iter() {
                            self.grid[*j as usize][*i as usize] += 1;
                        }
                    }
                }
            }
        }
        // Reset flashes.
        for row in self.grid.iter_mut() {
            for level in row.iter_mut() {
                if *level > 9 {
                    *level = 0;
                }
            }
        }
        flashed.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, f| acc + if *f { 1 } else { 0 })
        })
    }

    fn run_steps(&mut self, n: u64) {
        for _ in 0..n {
            self.step();
        }
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Dumbo Octopus");
    puzzle.set_part1("number of flashes", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let mut octopi = DumboOctopi::new(lines);
        octopi.run_steps(100);
        octopi.flashes
    });
    puzzle.set_part2("first synchronized step", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let mut octopi = DumboOctopi::new(lines);
        while octopi.step() != 100 {}
        octopi.steps
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
    fn test_examples() {
        let example: Vec<String> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let mut octopi1 = DumboOctopi::new(example.clone());
        octopi1.step();
        assert_eq!(octopi1.flashes, 0);
        octopi1.step();
        assert_eq!(octopi1.flashes, 35);
        octopi1.run_steps(8);
        assert_eq!(octopi1.flashes, 204);
        octopi1.run_steps(90);
        assert_eq!(octopi1.flashes, 1656);
        let mut octopi2 = DumboOctopi::new(example);
        while octopi2.step() != 100 {}
        assert_eq!(octopi2.steps, 195);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(1675);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(515);
    }
}
