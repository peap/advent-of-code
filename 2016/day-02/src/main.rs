use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_X: i32 = 3;
const MAX_Y: i32 = 3;

type Button = char;
type ButtonLocations = HashMap<Button, (usize, usize)>;
type MoveSet = Vec<String>;

type KeyPad = [[Button; MAX_X as usize]; MAX_Y as usize];

const KEYPAD1: KeyPad = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
];

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

fn get_code(keypad: KeyPad, start: Button, all_moves: MoveSet) -> String {
    let button_locations = get_button_locations(keypad);
    let mut buttons: Vec<Button> = Vec::new();
    for moveset in all_moves {
        let button = get_button(&button_locations, keypad, start, moveset);
        buttons.push(button);
    }
    buttons.iter().map(|x| x.to_string()).collect::<String>()
}

fn get_button_locations(keypad: KeyPad) -> ButtonLocations {
    let mut locations = HashMap::new();
    for (y, row) in keypad.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            locations.insert(button.clone(), (x, y));
        }
    }
    locations
}

fn get_button(button_locations: &ButtonLocations,
              keypad: KeyPad,
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
            x = if x >= MAX_X { MAX_X - 1 } else { x };
        }
        if dy < 0 {
            y = if y == 0 { 0 } else { y + dy }
        } else {
            y += dy;
            y = if y >= MAX_Y { MAX_Y - 1 } else { y };
        }
        button = keypad[y as usize][x as usize];
    }
    button
}

fn main() {
    let all_moves = load_moves("input.txt");
    let code = get_code(KEYPAD1, '5', all_moves);
    println!("The first code is {}.", code);
}
