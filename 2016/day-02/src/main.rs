use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type ButtonLocations = HashMap<i32, (i32, i32)>;

const MAX_X: i32 = 3;
const MAX_Y: i32 = 3;

const KEYPAD: [[i32; MAX_X as usize]; MAX_Y as usize] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];


fn get_button_locations() -> ButtonLocations {
    let mut locations = HashMap::new();
    for (y, row) in KEYPAD.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            locations.insert(button.clone(), (x as i32, y as i32));
        }
    }
    locations
}

fn load_moves(filename: &'static str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => lines.push(text),
            _ => ()
        }
    }
    lines
}

fn get_button(button_locations: &ButtonLocations, start: i32, moveset: String) -> i32 {
    let mut button = start;
    let mut old_button;
    let location = button_locations.get(&button).unwrap();
    let mut x = location.0;
    let mut y = location.1;
    for c in moveset.chars() {
        let (dx, dy) = match c {
            'U' => ( 0, -1),
            'D' => ( 0,  1),
            'L' => (-1,  0),
            'R' => ( 1,  0),
            _ => panic!("Unknown move, {}", c),
        };
        x += dx;
        x = if x < 0 { 0 } else { x };
        x = if x >= MAX_X { MAX_X - 1 } else { x };
        y += dy;
        y = if y < 0 { 0 } else { y };
        y = if y >= MAX_Y { MAX_Y - 1 } else { y };
        old_button = button;
        button = KEYPAD[y as usize][x as usize];
        println!("From {} we go {} to {} ({}, {}).", old_button, c, button, x, y);
    }
    button
}

fn main() {
    let all_moves = load_moves("input.txt");
    let mut buttons: Vec<i32> = Vec::new();
    let previous_button = 5;
    let button_locations = get_button_locations();
    for moveset in all_moves {
        let button = get_button(&button_locations, previous_button, moveset);      
        buttons.push(button);
    }
    let code: String = buttons.iter().map(|x| x.to_string()).collect();
    println!("The code is {}.", code);
}
