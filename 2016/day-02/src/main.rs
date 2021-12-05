use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Button = char;
type ButtonLocations = HashMap<Button, (usize, usize)>;
type Moves = String;
type MoveSet = Vec<Moves>;
type KeyPad = Vec<Vec<Button>>;

const XXX: Button = '*';

fn get_keypad_1() -> KeyPad {
    vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ]
}

fn get_keypad_2() -> KeyPad {
    vec![
        vec![XXX, XXX, '1', XXX, XXX],
        vec![XXX, '2', '3', '4', XXX],
        vec!['5', '6', '7', '8', '9'],
        vec![XXX, 'A', 'B', 'C', XXX],
        vec![XXX, XXX, 'D', XXX, XXX],
    ]
}

fn load_moves(filename: &'static str) -> MoveSet {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => lines.push(text),
            _ => (),
        }
    }
    lines
}

fn get_code(keypad: KeyPad, start: Button, all_moves: &MoveSet) -> String {
    let button_locations = get_button_locations(&keypad);
    let mut buttons: Vec<Button> = Vec::new();
    for moveset in all_moves {
        let button = follow_moves(&button_locations, &keypad, start, moveset);
        buttons.push(button);
    }
    buttons.iter().map(|x| x.to_string()).collect::<String>()
}

fn get_button_locations(keypad: &KeyPad) -> ButtonLocations {
    let mut locations = HashMap::new();
    for (y, row) in keypad.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            locations.insert(button.clone(), (x, y));
        }
    }
    locations
}

fn follow_moves(
    button_locations: &ButtonLocations,
    keypad: &KeyPad,
    start: Button,
    moveset: &Moves,
) -> Button {
    let max_x = keypad[0].len() as i32;
    let max_y = keypad.len() as i32;
    let mut button = start;
    let location = button_locations.get(&button).unwrap();
    let mut x = location.0 as i32;
    let mut y = location.1 as i32;
    let mut old_x;
    let mut old_y;
    for c in moveset.chars() {
        let (dx, dy) = match c {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Unknown move, {}", c),
        };
        old_x = x;
        old_y = y;

        if dx < 0 {
            x = if x == 0 { 0 } else { x + dx }
        } else {
            x += dx;
            x = if x >= max_x { max_x - 1 } else { x };
        }
        if dy < 0 {
            y = if y == 0 { 0 } else { y + dy }
        } else {
            y += dy;
            y = if y >= max_y { max_y - 1 } else { y };
        }
        if get_button(keypad, x, y) == XXX {
            x = old_x;
            y = old_y;
        }
        button = get_button(keypad, x, y);
    }
    button
}

fn get_button(keypad: &KeyPad, x: i32, y: i32) -> Button {
    keypad[y as usize][x as usize]
}

fn main() {
    let all_moves = load_moves("input.txt");
    let code1 = get_code(get_keypad_1(), '5', &all_moves);
    println!("The first code: {}", code1);
    let code2 = get_code(get_keypad_2(), '5', &all_moves);
    println!("The second code: {}", code2);
}
