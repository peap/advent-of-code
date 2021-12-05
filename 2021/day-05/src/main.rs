use lazy_static::lazy_static;
use regex::Regex;

use common::InputReader;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
}

struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

impl Line {
    fn from_str(s: &str) -> Line {
        let caps = LINE_RE.captures(s).unwrap();
        let x1 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y1 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x2 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y2 = caps.get(4).unwrap().as_str().parse().unwrap();
        Line {
            from: (x1, y1),
            to: (x2, y2),
        }
    }

    fn is_diagonal(&self) -> bool {
        self.from.0 != self.to.0 && self.from.1 != self.to.1
    }
}

struct Map {
    points: Vec<Vec<usize>>,
    #[allow(dead_code)]
    xmax: usize,
    #[allow(dead_code)]
    ymax: usize,
}

impl Map {
    fn new(lines: Vec<Line>) -> Map {
        // Find map size.
        let (mut xmax, mut ymax) = (0, 0);
        for l in lines.iter() {
            if l.from.0 > xmax {
                xmax = l.from.0;
            }
            if l.from.1 > ymax {
                ymax = l.from.1;
            }
            if l.to.0 > xmax {
                xmax = l.to.0;
            }
            if l.to.1 > ymax {
                ymax = l.to.1;
            }
        }

        // Create initial map of zeroes.
        let mut points = vec![vec![0; ymax + 1]; xmax + 1];

        // Draw lines on the map.
        for line in lines.iter() {
            Map::add_line(&mut points, line);
        }

        Map { points, xmax, ymax }
    }

    fn add_line(points: &mut Vec<Vec<usize>>, line: &Line) {
        if line.from.0 == line.to.0 {
            // horizontal
            let x = line.from.0;
            let mut ys = [line.from.1, line.to.1];
            ys.sort_unstable();
            for y in ys[0]..ys[1] + 1 {
                points[x][y] += 1
            }
        } else if line.from.1 == line.to.1 {
            // vertical
            let y = line.from.1;
            let mut xs = [line.from.0, line.to.0];
            xs.sort_unstable();
            #[allow(clippy::needless_range_loop)]
            for x in xs[0]..xs[1] + 1 {
                points[x][y] += 1
            }
        } else {
            // diagonal
            let xs = [line.from.0 as i64, line.to.0 as i64];
            let ys = [line.from.1 as i64, line.to.1 as i64];
            let xdelta = if xs[0] < xs[1] { 1 } else { -1 };
            let ydelta = if ys[0] < ys[1] { 1 } else { -1 };
            let (mut x, mut y) = (xs[0], ys[0]);
            for _ in 0..((xs[1] - xs[0]).abs() + 1) {
                points[x as usize][y as usize] += 1;
                x += xdelta;
                y += ydelta;
            }
        }
    }

    fn count_overlapping(&self, min: usize) -> usize {
        let mut overlapping = 0;
        for row in self.points.iter() {
            for point in row.iter() {
                if *point >= min {
                    overlapping += 1
                }
            }
        }
        overlapping
    }

    #[cfg(test)]
    fn print(&self) {
        for y in 0..self.ymax + 1 {
            for x in 0..self.xmax + 1 {
                let point = self.points[x][y];
                if point > 0 {
                    print!("{} ", point);
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

fn cloud_map(line_strings: Vec<String>, with_diagonals: bool) -> Map {
    let mut lines = vec![];
    for ls in line_strings.iter() {
        let line = Line::from_str(ls);
        if !line.is_diagonal() || with_diagonals {
            lines.push(line);
        }
    }
    Map::new(lines)
}

fn main() {
    let reader = InputReader::new("input.txt");
    let lines = reader.string_lines();
    let map1 = cloud_map(lines.clone(), false);
    let overlaps1 = map1.count_overlapping(2);
    println!("Part 1: overlapping points: {}", overlaps1);
    let map2 = cloud_map(lines, true);
    let overlaps2 = map2.count_overlapping(2);
    println!("Part 2: overlapping points: {}", overlaps2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines: Vec<String> = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        let map1 = cloud_map(lines.clone(), false);
        map1.print();
        let overlaps1 = map1.count_overlapping(2);
        assert_eq!(overlaps1, 5);
        let map2 = cloud_map(lines.clone(), true);
        println!();
        map2.print();
        let overlaps2 = map2.count_overlapping(2);
        assert_eq!(overlaps2, 12);
    }

    #[test]
    fn test_part1() {
        let reader = InputReader::new("input.txt");
        let lines = reader.string_lines();
        let map = cloud_map(lines.clone(), false);
        assert_eq!(map.count_overlapping(2), 5306);
    }

    #[test]
    fn test_part2() {
        let reader = InputReader::new("input.txt");
        let lines = reader.string_lines();
        let map = cloud_map(lines.clone(), true);
        assert_eq!(map.count_overlapping(2), 17787);
    }
}
