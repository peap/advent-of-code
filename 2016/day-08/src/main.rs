use regex::Regex;

use common::InputReader;

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        use Instruction::*;
        let rect_re = Regex::new(r"^rect ([0-9]+)x([0-9]+)$").unwrap();
        let rrow_re = Regex::new(r"^rotate row y=([0-9]+) by ([0-9]+)$").unwrap();
        let rcol_re = Regex::new(r"^rotate column x=([0-9]+) by ([0-9]+)$").unwrap();
        if rect_re.is_match(line) {
            let caps = rect_re.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            Rect(x, y)
        } else if rrow_re.is_match(line) {
            let caps = rrow_re.captures(line).unwrap();
            let y = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let amount = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            RotateRow(y, amount)
        } else if rcol_re.is_match(line) {
            let caps = rcol_re.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let amount = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            RotateCol(x, amount)
        } else {
            panic!("Could not understand instruction: {}", line)
        }
    }
}

struct Display {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        let mut pixels = Vec::new();
        for _ in 0..height {
            pixels.push(vec![false; width]);
        }
        Display {
            width,
            height,
            pixels,
        }
    }

    fn process(&mut self, instructions: &[Instruction]) {
        use Instruction::*;
        for instruction in instructions {
            match *instruction {
                Rect(x, y) => self.illuminate(x, y),
                RotateRow(y, a) => self.rotate_row(y, a),
                RotateCol(x, a) => self.rotate_col(x, a),
            }
        }
    }

    fn illuminate(&mut self, x: usize, y: usize) {
        for iy in 0..y {
            for ix in 0..x {
                self.pixels[iy][ix] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, amount: usize) {
        let mut new_row = vec![false; self.width];
        let row = &mut self.pixels[y];
        for (ix, pixel) in row.iter().enumerate() {
            let new_ix = (ix + amount) % self.width;
            new_row[new_ix] = *pixel;
        }
        for (ix, pixel) in new_row.iter().enumerate() {
            row[ix] = *pixel;
        }
    }

    fn rotate_col(&mut self, x: usize, amount: usize) {
        let mut new_col = vec![false; self.height];
        for (iy, row) in self.pixels.iter().enumerate() {
            let new_iy = (iy + amount) % self.height;
            new_col[new_iy] = row[x];
        }
        for (iy, pixel) in new_col.iter().enumerate() {
            self.pixels[iy][x] = *pixel;
        }
    }

    fn num_on(&self) -> u32 {
        let mut count = 0;
        for row in self.pixels.iter() {
            for light in row {
                if *light {
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        for _ in 0..(self.width + 2) {
            print!("-");
        }
        println!();
        for row in self.pixels.iter() {
            print!("|");
            for pixel in row {
                if *pixel {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }
        for _ in 0..(self.width + 2) {
            print!("-");
        }
        println!();
    }
}

fn main() {
    let lines = InputReader::new("input.txt").string_lines();
    let instructions: Vec<Instruction> = lines.iter().map(|l| Instruction::from_line(l)).collect();
    let mut display = Display::new(50, 6);
    display.process(&instructions);
    println!("Part 1: The display has {} lights on.", display.num_on());
    println!("Part 2: ...");
    display.print();
}

#[test]
fn test_part_1() {
    let lines = InputReader::new("input.txt").string_lines();
    let instructions: Vec<Instruction> = lines.iter().map(|l| Instruction::from_line(l)).collect();
    let mut display = Display::new(50, 6);
    display.process(&instructions);
    assert_eq!(display.num_on(), 123);
}
