use std::cmp::Ordering;
use std::ops::Range;

use common::{default_puzzle, Answer, InputReader, Puzzle};

#[derive(Clone)]
struct Probe {
    xpos: i64,
    ypos: i64,
    xvel: i64,
    yvel: i64,
    max_height: i64,
    xrange: Range<i64>,
    yrange: Range<i64>,
}

impl Probe {
    fn new(xrange: Range<i64>, yrange: Range<i64>) -> Self {
        Probe {
            xpos: 0,
            ypos: 0,
            xvel: 0,
            yvel: 0,
            max_height: 0,
            xrange,
            yrange,
        }
    }

    fn advance(&mut self) -> bool {
        self.xpos += self.xvel;
        self.ypos += self.yvel;
        if self.ypos > self.max_height {
            self.max_height = self.ypos;
        }
        self.xvel += match self.xvel.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };
        self.yvel -= 1;
        self.xrange.contains(&self.xpos) && self.yrange.contains(&self.ypos)
    }

    fn max_height(&self) -> i64 {
        let mut max_height = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                let mut probe = self.clone();
                probe.xvel = x;
                probe.yvel = y;
                while probe.ypos >= self.yrange.start && probe.xpos <= self.xrange.end {
                    if probe.advance() && probe.max_height > max_height {
                        max_height = probe.max_height;
                        break;
                    }
                }
            }
        }
        max_height
    }

    fn count_velocities(&self) -> u64 {
        let mut count = 0;
        for x in 0..1000 {
            for y in -1000..1000 {
                let mut probe = self.clone();
                probe.xvel = x;
                probe.yvel = y;
                while probe.ypos >= self.yrange.start && probe.xpos <= self.xrange.end {
                    if probe.advance() {
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }
}

fn part1(_: &InputReader) -> Answer {
    let probe = Probe::new(217..241, -126..-68);
    probe.max_height() as u64
}

fn part2(_: &InputReader) -> Answer {
    let probe = Probe::new(217..241, -126..-68);
    probe.count_velocities()
}

fn get_puzzle() -> Puzzle {
    let mut puzzle = default_puzzle!("Trick Shot");
    puzzle.set_part1(part1, "highest possible height");
    puzzle.set_part2(part2, "number of initial velocities");
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
        let probe = Probe::new(20..31, -10..-4);
        assert_eq!(probe.max_height(), 45);
        assert_eq!(probe.count_velocities(), 112);
    }

    #[test]
    fn test_part1() {
        get_puzzle().test_part1(7875);
    }

    #[test]
    fn test_part2() {
        get_puzzle().test_part2(2321);
    }
}
