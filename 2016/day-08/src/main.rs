extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Instruction {
    fn from_line(text: String) -> Instruction {
        use Instruction::*;
        let rect_re = Regex::new(r"^rect ([0-9]+)x([0-9]+)$").unwrap();
        let rrow_re = Regex::new(r"^rotate row y=([0-9]+) by ([0-9]+)$").unwrap();
        let rcol_re = Regex::new(r"^rotate column x=([0-9]+) by ([0-9]+)$").unwrap();
        if rect_re.is_match(&text) {
            let caps = rect_re.captures(&text).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            Rect(x, y)
        } else if rrow_re.is_match(&text) {
            let caps = rrow_re.captures(&text).unwrap();
            let y = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let amount = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            RotateRow(y, amount)
        } else if rcol_re.is_match(&text) {
            let caps = rcol_re.captures(&text).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let amount = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            RotateCol(x, amount)
        } else {
            panic!("Could not understand instruction: {}", &text)
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
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(false);
            }
            pixels.push(row);
        }
        Display {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    fn process(&mut self, instructions: &Vec<Instruction>) {
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
        let mut new_row: Vec<bool> = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            new_row.push(false);
        }
        let ref mut row = self.pixels[y];
        for (ix, pixel) in row.iter().enumerate() {
            let new_ix = (ix + amount) % self.width;
            new_row[new_ix] = *pixel;
        }
        for (ix, pixel) in new_row.iter().enumerate() {
            row[ix] = *pixel;
        }
    }

    fn rotate_col(&mut self, x: usize, amount: usize) {
        let mut new_col: Vec<bool> = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            new_col.push(false);
        }
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
        print!("\n");
        for row in self.pixels.iter() {
            print!("|");
            for pixel in row {
                if *pixel {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            print!("|\n");
        }
        for _ in 0..(self.width + 2) {
            print!("-");
        }
        print!("\n");
    }
}

fn load_instructions(filename: &'static str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(text) => instructions.push(Instruction::from_line(text)),
            Err(e) => panic!("Couldn't read a line from {}: {}", filename, e),
        }
    }
    instructions
}

fn main() {
    let instructions = load_instructions("input.txt");
    let mut display = Display::new(50, 6);
    display.process(&instructions);
    println!("Part 1: The display has {} lights on.", display.num_on());
    println!("Part 2: ...");
    display.print();
}

#[test]
fn test_part_1() {
    let instructions = load_instructions("input.txt");
    let mut display = Display::new(50, 6);
    display.process(&instructions);
    assert_eq!(display.num_on(), 123);
}
