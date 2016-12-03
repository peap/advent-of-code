use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Button = char;
type KeyPad = Vec<Vec<Button>>;
type ButtonLocations = HashMap<Button, (usize, usize)>;
type MoveSet = Vec<String>;

const MX: i32 = 3;
const MY: i32 = 3;

const KEYPAD: [[Button; MX as usize]; MY as usize] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
];

fn get_button_locations() -> ButtonLocations {
    let mut locations = HashMap::new();
    for (y, row) in KEYPAD.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            locations.insert(button.clone(), (x, y));
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

fn get_button(button_locations: &ButtonLocations,
              start: Button,
              moveset: String) -> Button {
    let mut button = start;
    let location = button_locations.get(&button).unwrap();
    let mut x = location.0 as i32;
    let mut y = location.1 as i32;
    for c in moveset.chars() {
        let (dx, dy) = match c {
            'U' => ( 0, -1),
            'D' => ( 0,  1),
            'L' => (-1,  0),
            'R' => ( 1,  0),
            _ => panic!("Unknown move, {}", c),
        };

        if dx < 0 {
            x = if x == 0 { 0 } else { x + dx }
        } else {
            x += dx;
            x = if x >= MX { MX - 1 } else { x };
        }
        if dy < 0 {
            y = if y == 0 { 0 } else { y + dy }
        } else {
            y += dy;
            y = if y >= MY { MY - 1 } else { y };
        }
        button = KEYPAD[y as usize][x as usize];
    }
    button
}

fn get_code(keypad: KeyPad, start: Button, moveset: MoveSet) -> String {
    "".to_string()
}

fn main() {
    let all_moves = load_moves("input.txt");
    let mut buttons: Vec<Button> = Vec::new();
    let previous_button = '5';
    let button_locations = get_button_locations();
    for moveset in all_moves {
        let button = get_button(&button_locations, previous_button, moveset);      
        buttons.push(button);
    }
    let code: String = buttons.iter().map(|x| x.to_string()).collect();
    println!("The code is {}.", code);
}
