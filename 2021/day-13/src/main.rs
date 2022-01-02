use std::collections::VecDeque;

use common::{default_puzzle, Puzzle};

struct Fold {
    x_axis: bool,
    value: usize,
}

impl Fold {
    fn new(line: &str) -> Self {
        // "fold along x=1"
        // "fold along y=1"
        let mut splits = line.split(' ').nth(2).unwrap().split('=');
        let x_axis = splits.next().unwrap() == "x";
        let value: usize = splits.next().unwrap().parse().unwrap();
        Fold { x_axis, value }
    }
}

struct Paper {
    grid: Vec<Vec<bool>>,
    folds: VecDeque<Fold>,
}

impl Paper {
    fn new(lines: Vec<String>) -> Self {
        let mut dots: Vec<(usize, usize)> = vec![];
        let mut folds: VecDeque<Fold> = VecDeque::new();
        let mut iter = lines.iter();
        let empty = "".to_string();
        let mut line = iter.next().unwrap_or(&empty);
        // Coordinates of points in the grid.
        while !line.is_empty() {
            let mut point = line.split(',');
            let x = point.next().unwrap().parse().unwrap();
            let y = point.next().unwrap().parse().unwrap();
            dots.push((x, y));
            line = iter.next().unwrap_or(&empty);
        }
        // Folding instructions; the first fold over each axes dictates the size of the grid.
        let mut width: usize = 0;
        let mut height: usize = 0;
        line = iter.next().unwrap_or(&empty);
        while !line.is_empty() {
            let fold = Fold::new(line);
            if fold.x_axis {
                let implied_width = 2 * fold.value + 1;
                if implied_width > width {
                    width = implied_width;
                }
            } else {
                let implied_height = 2 * fold.value + 1;
                if implied_height > height {
                    height = implied_height;
                }
            }
            folds.push_back(fold);
            line = iter.next().unwrap_or(&empty);
        }
        // Make the grid.
        let mut grid = vec![vec![false; width]; height];
        for (x, y) in dots.into_iter() {
            grid[y][x] = true;
        }
        Paper { grid, folds }
    }

    fn fold(&mut self) -> bool {
        if self.folds.is_empty() {
            return false;
        }
        let fold = self.folds.pop_front().unwrap();
        let val = fold.value;
        if fold.x_axis {
            // Fold left across x=<val>.
            for row in self.grid.iter_mut() {
                let mut opposites: VecDeque<bool> = VecDeque::new();
                for _ in 0..val {
                    opposites.push_back(row.pop().unwrap());
                }
                row.pop(); // The middle column.
                for point in row.iter_mut() {
                    let removed = opposites.pop_front().unwrap();
                    *point = *point || removed;
                }
            }
        } else {
            // Fold up across y=<val>.
            let mut opposites: VecDeque<Vec<bool>> = VecDeque::new();
            for _ in 0..val {
                opposites.push_back(self.grid.pop().unwrap());
            }
            self.grid.pop(); // The middle line.
            for row in self.grid.iter_mut() {
                let removed = opposites.pop_front().unwrap();
                for (x, point) in row.iter_mut().enumerate() {
                    *point = *point || removed[x];
                }
            }
        }
        true
    }

    fn fold_all(&mut self) {
        while self.fold() {}
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for point in row.iter() {
                let c = if *point { '■' } else { '·' };
                print!("{}", c);
            }
            println!();
        }
    }

    fn count_dots(&self) -> u64 {
        let mut count = 0;
        for row in self.grid.iter() {
            count += row.iter().filter(|&&d| d).count() as u64;
        }
        count
    }
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Transparent Origami");
    puzzle.set_part1("dots after first fold", |reader| {
        let lines: Vec<String> = reader.parsed_lines();
        let mut paper = Paper::new(lines);
        paper.fold();
        paper.count_dots()
    });
    puzzle.set_part2(
        "see print-out above (FJAHJGAH); answer is not this value",
        |reader| {
            let lines: Vec<String> = reader.parsed_lines();
            let mut paper = Paper::new(lines);
            paper.fold_all();
            paper.print();
            paper.count_dots()
        },
    );
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
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();
        let mut paper = Paper::new(lines);
        paper.fold();
        assert_eq!(paper.count_dots(), 17);
        paper.fold_all();
        paper.print();
        assert_eq!(paper.count_dots(), 16);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(785);
    }

    #[test]
    fn test_part2() {
        // Actual answer is "FJAHJGAH".
        get_puzzle().test_part2(98);
    }
}
